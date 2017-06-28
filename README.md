car
---

Cody's Archive Reader

* Alternative to GNU Tar.
* Files produced by CAR can be read by TAR, and vice versa. 
* Has a different argument layout then *traditional* TAR which I feel is more readable.
* Supports extracting/listing with regex filters
* File sizes when listing is _always_ human readable.
* Update/Diff/Concatenate/Append not supported

### How to install:
1. Install Rust and Cargo
2. Ensure you have a valid c compiler (Microsoft Visual C compiler works at least VC2014)
3. Clone this directory `git clone https://github.com/valarauca/car`
4. `cargo build --release`
5. The executable will be located in `car/target/release`


### Usage:

This does not support the legacy, unix, or GNU flag styles. In the true spirit of TAR
I've created **MY OWN** flag systel. Which is based on sub commands so it should be
fairly easy to pick up a few examples

### Examples:

Please see `--help` for this. The man pages are mostly compiled into the library.

