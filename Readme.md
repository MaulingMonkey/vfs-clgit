# [vfs]-[clgit]: Virtual FileSystem abstractions via Command Line GIT

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/vfs-clgit.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/vfs-clgit)
[![crates.io](https://img.shields.io/crates/v/vfs-clgit.svg)](https://crates.io/crates/vfs-clgit)
[![docs.rs](https://docs.rs/vfs-clgit/badge.svg)](https://docs.rs/vfs-clgit)
[![%23![forbid(unsafe_code)]](https://img.shields.io/github/search/MaulingMonkey/vfs-clgit/unsafe%2bextension%3Ars?color=green&label=%23![forbid(unsafe_code)])](https://github.com/MaulingMonkey/vfs-clgit/search?q=forbid%28unsafe_code%29+extension%3Ars)
[![rust: 1.32.0+](https://img.shields.io/badge/rust-1.32.0%2B-yellow.svg)](https://gist.github.com/MaulingMonkey/c81a9f18811079f19326dac4daa5a359#minimum-supported-rust-versions-msrv)
[![License](https://img.shields.io/crates/l/vfs-clgit.svg)](https://github.com/MaulingMonkey/vfs-clgit)
[![Build Status](https://travis-ci.com/MaulingMonkey/vfs-clgit.svg?branch=master)](https://travis-ci.com/MaulingMonkey/vfs-clgit)
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/vfs-clgit/status.svg)](https://deps.rs/repo/github/MaulingMonkey/vfs-clgit) -->

Currently this just bridges [vfs] and [git] via [clgit].
Alternate VFS abstractions may be added in the future.



<h2 name="features">Features</h2>

| Feature       | Description |
| ------------- | ----------- |
| **default**   |
| vfs04         | [vfs] = "[0.4.x](http://docs.rs/vfs/0.4)" interop



<h2 name="unsafe-code">Unsafe Code</h2>

Crate uses <code>[#![forbid(unsafe_code)]](https://github.com/MaulingMonkey/vfs-clgit/search?q=forbid%28unsafe_code%29+extension%3Ars)</code>.
However, indirect dependencies do contain some `unsafe` - including, but perhaps not limited to:

| crate     | version |
| --------- | ------- |
| syn       | 1.0.40



<h2 name="msrv">MSRV (Minimum Supported Rust Version)</h2>

Currently 1.32.0...ish.
*   [vfs] 0.4.0 has a [MSRV of 1.32.0](https://github.com/manuel-woelker/rust-vfs/blob/c34f4ca/README.md#040-2020-08-13).
    However, it has no clear policy for when MSRV can be bumped.
*   Unpinned dependencies may break this



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



<h2 name="license">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.



[vfs]:          https://lib.rs/crates/vfs
[clgit]:        https://lib.rs/crates/clgit
[git]:          https://git-scm.com/
