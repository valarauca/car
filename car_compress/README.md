car-compress
---

This is the core compression suite used in the car file archive utility. 

The library is seperated out as it provides abstractions across many
rust compression utilities, and abstracts away the setup/tear down
of a compression algorithm. 

As well as detecting _what_ archive is being decompressed. 

