car
---

Cody's Archive Reader

This is an alternative to `tar` which supports _mostly_ similiar CLI
options... This is a work in progress `tar` does _a lot_. Currently it
is compatible to tar and supports most the _main stream_ formats which are
`gzip`, `xz`, `bzip2`, and `lzw`.

`car` has support for new tools like `brotli`, `zstd`, `lz4`, and `snappy`
built into itself. `zstd`'s default setting is rather nice offering _higher_
(YMMV) compression ratios then `gzip -9` at ~10x the speed. While its fast
setting can offers about ~2% improvement over `gzip -1` with a ~3.5x speed
increase (nearly 200MB/s). 


