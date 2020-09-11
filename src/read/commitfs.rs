use std::fmt::{self, Debug, Formatter};
use std::io::{self, Read, Seek, SeekFrom};
use std::sync::Arc;



/// A read-only filesystem for a single git [Commit](git::Commit)
///
/// # Example
///
/// ```rust
/// # use vfs_clgit::*;
/// # use vfs04::*;
/// let commitfs = CommitFS::new(".", "dcb6ca6b041186b6aeb17b9bce48ce2ae112b7dc").unwrap();
/// let commitfs = VfsPath::new(commitfs);
/// // ...
/// ```
pub struct CommitFS {
    src:    Arc<git::RepositoryCache>,
    commit: Arc<git::Commit>,
}

impl CommitFS {
    /// Create a filesystem over a commit.
    pub fn new(repository: impl git::TryIntoSharedRepositoryCache, commit: impl TryIntoCommitHash) -> io::Result<Self> {
        let src = repository.try_into_src()?;
        let commit = src.commit(&commit.try_into_commit_hash()?)?;
        Ok(Self { src, commit })
    }

    fn hash_for_path(&self, path: &str) -> io::Result<git::unknown::Hash> {
        let path = path.trim_matches(is_path_sep);

        let mut current = self.commit.tree.typeless();
        if path.is_empty() { return Ok(current) }

        for component in path.split(is_path_sep) {
            current = self.src
                .tree(&current.cast()).map_err(|_| io::ErrorKind::NotFound)?
                .entries.get(component).ok_or(io::ErrorKind::NotFound)?
                .hash.clone();
        }
        Ok(current)
    }
}

impl Debug for CommitFS {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "CommitFS(\"{}\")", self.commit.hash)
    }
}



/// Try to convert into a [commit::Hash], on pain of [std::io::Error].  Implemented for:<br>
/// &[str], \(&\)[String], &\[[u8]\], \(&\)[Vec]\<[u8]\>, \(&\)[commit::Hash], \(&\)[unknown::Hash]
///
/// [commit::Hash]:     https://docs.rs/clgit/0.1/clgit/commit/type.Hash.html
/// [unknown::Hash]:    https://docs.rs/clgit/0.1/clgit/unknown/type.Hash.html
pub trait TryIntoCommitHash {
    /// Try to convert into a [commit::Hash], on pain of [std::io::Error].
    ///
    /// [commit::Hash]:     https://docs.rs/clgit/0.1/clgit/commit/type.Hash.html
    fn try_into_commit_hash(self) -> io::Result<git::commit::Hash>;
}

impl TryIntoCommitHash for &str                 { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(git::commit::Hash::from_str(self)?) } }
impl TryIntoCommitHash for  String              { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(git::commit::Hash::from_str(self.as_str())?) } }
impl TryIntoCommitHash for &String              { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(git::commit::Hash::from_str(self.as_str())?) } }
impl TryIntoCommitHash for &[u8]                { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(git::commit::Hash::from_bytes(self)?) } }
impl TryIntoCommitHash for  Vec<u8>             { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(git::commit::Hash::from_bytes(&self[..])?) } }
impl TryIntoCommitHash for &Vec<u8>             { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(git::commit::Hash::from_bytes(&self[..])?) } }
impl TryIntoCommitHash for  git::commit::Hash   { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(self) } }
impl TryIntoCommitHash for &git::commit::Hash   { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(self.clone()) } }
impl TryIntoCommitHash for  git::unknown::Hash  { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(self.cast()) } }
impl TryIntoCommitHash for &git::unknown::Hash  { fn try_into_commit_hash(self) -> io::Result<git::commit::Hash> { Ok(self.cast()) } }



#[cfg(feature = "vfs04")] mod vfs04 {
    use super::*;
    use ::vfs04::*;
    use std::io::Write;

    impl FileSystem for CommitFS {
        fn read_dir(&self, path: &str) -> VfsResult<Box<dyn Iterator<Item = String>>> {
            let hash = self.hash_for_path(path)?;
            let tree = self.src.tree(&hash.cast())?;
            let mut files = Vec::new();
            for (name, _entry) in tree.entries.iter() {
                files.push(name.as_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Directory contains non-UTF8 paths"))?.to_owned());
            }
            Ok(Box::new(files.into_iter()))
        }

        fn open_file(&self, path: &str) -> VfsResult<Box<dyn SeekAndRead>> {
            let hash = self.hash_for_path(path)?;
            Ok(Box::new(ReadFakeSeek(self.src.repository.cat_file_blob(&hash.cast())?)))
        }

        fn metadata(&self, path: &str) -> VfsResult<VfsMetadata> {
            let hash = self.hash_for_path(path)?;
            let t = self.src.repository.cat_file_type(&hash)?;
            match t {
                git::FileType::Blob     => Ok(VfsMetadata { file_type: VfsFileType::File,      len: self.src.repository.cat_file_size(&hash.cast())? }),
                git::FileType::Tree     => Ok(VfsMetadata { file_type: VfsFileType::Directory, len: 0 }),
                git::FileType::Commit   => Err(VfsError::FileNotFound { path: path.into() }),
                _                       => Err(VfsError::FileNotFound { path: path.into() }),
            }
        }

        fn exists(&self, path: &str) -> bool {
            self.hash_for_path(path).is_ok()
        }

        // these all involve writing, which CommitFS doesn't support
        fn create_dir   (&self, _path: &str)            -> VfsResult<()>                { Err(VfsError::NotSupported) }
        fn create_file  (&self, _path: &str)            -> VfsResult<Box<dyn Write>>    { Err(VfsError::NotSupported) }
        fn append_file  (&self, _path: &str)            -> VfsResult<Box<dyn Write>>    { Err(VfsError::NotSupported) }
        fn remove_file  (&self, _path: &str)            -> VfsResult<()>                { Err(VfsError::NotSupported) }
        fn remove_dir   (&self, _path: &str)            -> VfsResult<()>                { Err(VfsError::NotSupported) }
        fn copy_file    (&self, _src: &str, _dst: &str) -> VfsResult<()>                { Err(VfsError::NotSupported) }
        fn move_file    (&self, _src: &str, _dst: &str) -> VfsResult<()>                { Err(VfsError::NotSupported) }
        fn move_dir     (&self, _src: &str, _dst: &str) -> VfsResult<()>                { Err(VfsError::NotSupported) }
    }
}



struct ReadFakeSeek<IO: Read>(IO);
impl<IO: Read> Read for ReadFakeSeek<IO> { fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { self.0.read(buf) } }
impl<IO: Read> Seek for ReadFakeSeek<IO> { fn seek(&mut self, _: SeekFrom) -> io::Result<u64> { Err(io::Error::new(io::ErrorKind::Other, "Seek not implemented for git streams")) } }

fn is_path_sep(ch: char) -> bool { ch == '/' || ch == '\\' }
