car
---

Cody's Archive Reader

This is an alternative to `tar` which supports _mostly_ similiar CLI
options... This is a work in progress `tar` does _a lot_. Currently it
is compatible to tar and supports most the _main stream_ formats which are
`gzip`, `xz`, `bzip2`, and `lzw`.

`car` has support for new tools like `brotli`, `zstd`, `lz4`, and `snappy`
built into itself.

###How to install:
1. Install Rust and Cargo
2. Ensure you have a valid c compiler (Microsoft Visual C compiler works at least VC2014)
3. Clone this directory `git clone https://github.com/valarauca/car`
4. `cargo build --release`
5. The executable will be located in `car/target/release`


###Usage:

This does not suppor the legacy, unix, or GNU flag styles. In the true spirit of TAR
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

