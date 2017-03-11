Algorithms
---

Compression algorithms are included in this directory just to be safe.

This file exists to list credit/thanks/changes.

#LZW

* Credit: [nwin](https://github.com/nwin)]
* Repo: [link](https://github.com/nwin/lzw)]
* Changes:
  * 2017-03-11: Updated `lzw.rs` to no longer import `std::io::Write` just to a build warning
* License: MIT/Apache-2.0 (Dual license)

#Gzip
* Credit: [Takeru Ohta](https://github.com/sile)
* Repo: [link](https://github.com/sile/libflate)
* Changes: NONE
* License: MIT

#Snappy
* Credit [Andrew Gallant](https://github.com/burntshushi)
* Repo: [link](https://github.com/burntsushi/rust-snappy)
* Changes: NONE
* License: BSD-3 Clause

#Bzip2
* Credit: [Alex Crichton](https://github.com/alexcrichton)
* Repo: [link](https://github.com/alexcrichton/bzip2-rs)
* Changes: NONE
* License: MIT/Apache-2.0 (Dual License)

#XZ
* Credit: [Alex Crichton](https://github.com/alexcrichton)
* Repo: [link](https://github.com/alexcrichton/xz2-rs)
* Changes:
  * 2017-03-02: Updated VC build to use VC2014 tooling about ~14 microsoft related files modified
* License: MIT/Apache-2.0 (Dual License)

#Brotli
* Credit: [Alex Crichton](https://github.com/alexcrichton)
* Repo: [link](https://github.com/alexcrichton/brotli2-rs)
* Changes:
  * 2017-03-01: Modified constructors [Pull Request](https://github.com/alexcrichton/brotli2-rs/pull/6)
  * 2017-03-11: Modified `CompresssionMode` to have the `Clone`, `Copy`, and `Debug` traits. Included in above PR
* License: MIT/Apache-2.0 (Dual License)

#Lz4
* Credit: [Artem V Navrotskiy](https://github.com/bozaro)
* Repo: [link](https://github.com/bozaro/lz4-rs)
* Changes:
  * 2017-03-01: Updated `lz4-rs/lz4-sys/src/lib.rs` due to type check error in test line ~347 (deleted test) [bug_report](https://github.com/bozaro/lz4-rs/issues/19)
  * 2017-03-11: Updated several `EncoderBuilder` enumerators to allow for `Clone`, `Copy`, and `Debug` traits
* License: MIT

#Tar
* Credit: [Alex Crichton](https://github.com/alexcrichton)
* Repo: [link](https://github.com/alexcrichton/tar-rs)
* Changes: NONE
* License: MIT/Apache-2.0 (Dual License)

#LZH

**NOT SUPPORTED**

#LZOP

**NOT SUPPORTED**

#Zstd
* Credit: [Alexandre Bury](https://github.com/gyscos)
* Repo: [link](https://github.com/gyscos/zstd-rs)
* Changes: NONE
* License: MIT

