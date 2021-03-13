# bset
[![Crates.io status](https://badgen.net/crates/v/bset)](https://crates.io/crates/bset)
[![Docs](https://docs.rs/bset/badge.svg)](https://docs.rs/bset)

Fast and compact sets of bytes and ASCII characters,
useful for searching, parsing and determining membership of a given byte
in the given set.
They don't use any allocation, nor even any `std` features.
In fact, all of the provided functions are `const`, so they can be freely
constructed at the compile time

## Sets
This crate exports two set types - `ByteSet` for a set of general bytes
and `AsciiSet` for a set restricted to the range of ASCII characters.
The is two times smaller in the memory, but comes with a slight
performance trade-off in having to check whether the given character
belongs to the ASCII range.
```rust
use ascii_set::AsciiSet;

const OP: AsciiSet = AsciiSet::new().add_bytes(b"+-*/%&|^");
assert!(OP.contains(b'%'));
```
The sets are implemented as an array of pointer-sized bit masks.

## Stack of sets
Inspired by [this article](https://maciej.codes/2020-04-19-stacking-luts-in-logos.html)
by Maciej Hirsz, this crate provides a way to stack multiple sets into one
structure to gain even more performance.
To not slow it down, sets are "obtained" from the given stack at the type
level. For this, the module `bits` contains types `B0`, `B1`, ..., `B7`
representing indices of a set in the stack.
Because `const fn`s currently don't support generic functions, the sets
are indexed by the order they were added to the stack.
Type aliases can be used to identify the sets within the stack:
```rust
	use ascii_set::{bits::*, ByteSet, ByteStack};

const BYTE_STACK: ByteStack<B3> = ByteStack::new()
    .add_set(ByteSet::DIGITS)
    .add_set(ByteSet::ALPHABETIC)
    .add_set(ByteSet::new().add_bytes(b"+-*/%&|^"));
type Digits = B0;
type Alphabetic = B1;
type Operations = B2;
assert!(BYTE_STACK.contains::<Operations>(b'%'));
```
Again, there are two versions, `ByteStack` for all bytes and `AsciiStack`
restricted to the ASCII range. Benchmarks show that testing the set membership
is about 20% faster with stacked sets. They come with 8 times larger
memory size (128/256 bytes vs. 16/32), which does not increase with the stacks
added, so when 8 sets (the maximum number) are used in one stack,
the memory size is equivalent.

## Benchmarks
Stacked full byte set version consistently outperforms both `match`ing and `std`
`is_ascii_*` functions. For some simple sets, the set version can be a bit slower.

Alphanumeric characters:
```
test alnum_ascii_set        ... bench:       1,051 ns/iter (+/- 48) = 974 MB/s
test alnum_ascii_stack      ... bench:         801 ns/iter (+/- 33) = 1278 MB/s
test alnum_byte_set         ... bench:         839 ns/iter (+/- 50) = 1220 MB/s
test alnum_byte_stack       ... bench:         620 ns/iter (+/- 33) = 1651 MB/s
test alnum_is_alnum         ... bench:       1,574 ns/iter (+/- 70) = 650 MB/s
test alnum_match            ... bench:       1,573 ns/iter (+/- 86) = 650 MB/s
```

Alphabetic characters:
```
test letter_ascii_set       ... bench:       1,027 ns/iter (+/- 42) = 997 MB/s
test letter_ascii_stack     ... bench:         943 ns/iter (+/- 45) = 1085 MB/s
test letter_byte_set        ... bench:         839 ns/iter (+/- 34) = 1220 MB/s
test letter_byte_stack      ... bench:         619 ns/iter (+/- 29) = 1654 MB/s
test letter_is_alphabetic   ... bench:         820 ns/iter (+/- 42) = 1248 MB/s
test letter_match           ... bench:         825 ns/iter (+/- 36) = 1241 MB/s
```

Lowercase characters:
```
test lowercase_ascii_set    ... bench:       1,197 ns/iter (+/- 52) = 855 MB/s
test lowercase_ascii_stack  ... bench:         893 ns/iter (+/- 45) = 1146 MB/s
test lowercase_byte_set     ... bench:         890 ns/iter (+/- 44) = 1150 MB/s
test lowercase_byte_stack   ... bench:         451 ns/iter (+/- 14) = 2270 MB/s
test lowercase_is_lowercase ... bench:         752 ns/iter (+/- 33) = 1361 MB/s
test lowercase_match        ... bench:         771 ns/iter (+/- 67) = 1328 MB/s
```

URI reserved characters (per RFC 3986, section 2.2):
```
test uri_ascii_set          ... bench:       1,243 ns/iter (+/- 87) = 823 MB/s
test uri_ascii_stack        ... bench:         887 ns/iter (+/- 103) = 1154 MB/s
test uri_byte_set           ... bench:         905 ns/iter (+/- 84) = 1131 MB/s
test uri_byte_stack         ... bench:         610 ns/iter (+/- 35) = 1678 MB/s
test uri_match              ... bench:       1,294 ns/iter (+/- 45) = 791 MB/s
```

License: MIT
