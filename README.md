# squashfs-rust

A library to read squashfs archives. This is basically a rough port of [my Go library](https://github.com/CalebQ42/squashfs) as a way to learn Rust. Since this is my way of learning Rust, it's probably pretty terrible overall and any suggestions and PRs are welcome.

## Features

- Read the squashfs's superblock
- Most compression types should be supported
  - No LZO support. This is mainly an issue with there being only a few libraries, none of which have the needed capabilities.
  - As of yet, nothing has been tested. In particular, the Xz/Lzma library I'm using has no mention of Lzma filters, but it's using the C library so it _probably_ works.
- Decoding Metadata blocks.
  - Maybe even correctly.
- Decoding inodes
- Decoding directory trees
  - This doesn't seem to be correct just yet.

## TODO

- Decode data blocks
- Extract files
- Make an "easy" to use API
