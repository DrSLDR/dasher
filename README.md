# Dasher

_A **D**irectory H**asher**_

`dasher` is a small utility intended to allow you to condense the "status" of an entire
directory tree down to a single hash digest. Thus, you can tell that _something_ has
changed in the tree (but not _what_ has changed) if the hash has changed.

## Installation

`dasher` is easiest to install via Cargo

```
$ cargo install dasher
```

You can, of course, also clone this repository and `cargo build`/`cargo run` the code
that way.

## Usage

`dasher` currently does not have a CLI, but it is in the works.

## Hashing scheme

The hashing scheme is, in essence, generating a [Merkle
tree](https://en.wikipedia.org/wiki/Merkle_tree), but with extra steps. Each node in the
directory tree has its name hashed, then its contents, then those hashes are
concatenated with a separator byte based on the node's type, and that data is hashed
again to generate the node's hash. This process is repeated, from the bottom up in the
directory tree, until all nodes have been hashed and a final hash for the entire
directory can be returned.

For normal **files**, the node hash is simply:
```
hash(hash(name) + byte + hash(content))
```

For **directories**, the node hash includes arbitrarily many content hashes, one per
sub-node:
```
hash(hash(name) + byte + hash(content_1) + byte + hash(content_2) + ... + byte + hash(content_n))
```

Finally, for **symlinks**, the link isn't followed. Instead, the content hash is the
hash of the path to the file the link points to.
```
hash(hash(name) + byte + hash(path))
```

Traversal of the directory is _not_ recursive --- rather, the process starts with the
leaves in the lexicographically "first" directory. For example, in the directory
```
.git
├── COMMIT_EDITMSG
├── config
├── info
│   └── exclude
└── logs
    ├── HEAD
    └── refs
        ├── heads
        │   ├── add-cli
        │   ├── dh-main
        │   └── main
        └── remotes
            └── origin
                └── main

```
the first item to be hashed would be `info/exclude`, followed by the directory hash of
the `info` directory. After that, `logs/refs/heads/*` would be hashed, then
`logs/refs/heads/remotes/origin/main`, then `logs/refs/heads/remotes/origin` as a
directory, then `logs/refs/heads/remotes` as a directory, then finally climbing back up
to hash `logs/refs`, since both its sub-directories have been hashed.

In a way, I guess you could consider this as being recursive, but it is not implemented
recursively.

## License

`dasher` is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Is it any good?

[yes.](https://news.ycombinator.com/item?id=3067434)
