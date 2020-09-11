use vfs_clgit::CommitFS;
use vfs04::*;

fn main() {
    let commitfs = CommitFS::new(".", "dcb6ca6b041186b6aeb17b9bce48ce2ae112b7dc").expect("Unable to read commit");
    let commitfs = VfsPath::new(commitfs);
    dump(&commitfs);
}

fn dump(path: &VfsPath) {
    let meta = path.metadata().unwrap();
    match meta.file_type {
        VfsFileType::File => {
            let contents = path.read_to_string().unwrap();
            assert_eq!(contents.as_bytes().len() as u64, meta.len);

            println!("{}", path.as_str());
            println!("{:=>1$}", "========", path.as_str().len());
            println!("{}", contents);
            println!();
            println!();
            println!();
        },
        VfsFileType::Directory => {
            for path in path.read_dir().expect("Unable to read directory") {
                dump(&path);
            }
        },
    }
}
