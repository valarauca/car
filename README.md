car
---

Cody's Archive Reader

* Alternative to GNU Tar.
* Files produced by CAR can be read by TAR, and vice versa. 
* Has a different argument layout then *traditional* TAR which I feel is more readable.
* Supports extracting/listing with regex filters
* File sizes when listing is _always_ human readable.
* Update/Diff/Concatenate/Append not supported

###How to install:
1. Install Rust and Cargo
2. Ensure you have a valid c compiler (Microsoft Visual C compiler works at least VC2014)
3. Clone this directory `git clone https://github.com/valarauca/car`
4. `cargo build --release`
5. The executable will be located in `car/target/release`


###Usage:

This does not support the legacy, unix, or GNU flag styles. In the true spirit of TAR
I've created **MY OWN** flag systel. Which is based on sub commands so it should be
fairly easy to pick up a few examples

###Examples:

```bash
#Creating archives:

car create gzip -f dir1/ dir2/ dir3/ -o bundle.tar.gz
car create xz -f stuff.tar -o stuff.tar.gz
#Some algorithms support --fast and --slow (for a stand in to -0/-9)
car create lz4 -f dir1 -o dir1.tar.lz4 --fast


#Listing Contents:

car list -f bundle.tar.gz
#You can list additional things by providing flags
car list -f bundle.tar.gz --username --groupname --size
#You can even filter the list view via regex
car list -f bundle.tar.gz --regex='.*exe' --size

#Extracting:

car extract -f bundle.tar.gz -o where/to/extract
#Extracting with regex
car extract -f bundle.tar.gz --regex '.*c'
```

