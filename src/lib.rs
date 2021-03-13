//! Fast and compact sets of bytes or ASCII characters,
//! useful for searching, parsing and determining membership of a given byte
//! in the given set.
//! They don't use any allocation, nor even any `std` features.
//! In fact, all of the provided functions are `const`, so they can be freely
//! constructed at the compile time
//!
//! # Sets
//! This crate exports two set types - `ByteSet` for a set of general bytes
//! and `AsciiSet` for a set restricted to the range of ASCII characters.
//! The is two times smaller in the memory, but comes with a slight
//! performance trade-off in having to check whether the given character
//! belongs to the ASCII range.
//! ```
//! use bset::AsciiSet;
//!
//! const OP: AsciiSet = AsciiSet::new().add_bytes(b"+-*/%&|^");
//! assert!(OP.contains(b'%'));
//! ```
//! The sets are implemented as an array of pointer-sized bit masks.
//!
//! # Stack of sets
//! Inspired by [this article](https://maciej.codes/2020-04-19-stacking-luts-in-logos.html)
//! by Maciej Hirsz, this crate provides a way to stack multiple sets into one
//! structure to gain even more performance.
//! To not slow it down, sets are "obtained" from the given stack at the type
//! level. For this, the module `bits` contains types `B0`, `B1`, ..., `B7`
//! representing indices of a set in the stack.
//! Because `const fn`s currently don't support generic functions, the sets
//! are indexed by the order they were added to the stack.
//! Type aliases can be used to identify the sets within the stack:
//! ```
//! use bset::{bits::*, ByteSet, ByteStack};
//! 
//! const BYTE_STACK: ByteStack<B3> = ByteStack::new()
//!     .add_set(ByteSet::DIGITS)
//!     .add_set(ByteSet::ALPHABETIC)
//!     .add_set(ByteSet::new().add_bytes(b"+-*/%&|^"));
//! type Digits = B0;
//! type Alphabetic = B1;
//! type Operations = B2;
//! assert!(BYTE_STACK.contains::<Operations>(b'%'));
//! ```
//! Again, there are two versions, `ByteStack` for all bytes and `AsciiStack`
//! restricted to the ASCII range. Benchmarks show that testing the set membership
//! is about 20% faster with stacked sets. They come with 8 times larger
//! memory size (128/256 bytes vs. 16/32), which does not increase with the stacks
//! added, so when 8 sets (the maximum number) are used in one stack,
//! the memory size is equivalent.
#![no_std]
#![warn(missing_docs)]
mod bit;
/// Types that denote the position of a byte set within a byte stack.
pub mod bits {
    pub use crate::bit::{B0, B1, B2, B3, B4, B5, B6, B7};
}
use bit::Bit;
use bits::*;
use core::marker::PhantomData;
use core::ops::RangeInclusive;

type Chunk = usize;
/// Range of ASCII characters.
pub const ASCII_RANGE_LEN: usize = 0x80;
/// Size of one chunk of the mask in the implementation of byte sets.
pub const CHUNK_SIZE: usize = core::mem::size_of::<Chunk>();
/// Number of bytes in one chunk of the mask.
pub const BITS_PER_CHUNK: usize = 8 * CHUNK_SIZE;
/// Number of chunks in the ASCII set.
pub const CHUNKS: usize = ASCII_RANGE_LEN / BITS_PER_CHUNK;

/// A compact set of bytes.
/// Only particular instances - `AsciiSet` and `ByteSet` can be constructed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnyByteSet<const N: usize> {
    mask: [Chunk; N],
}

/// A compact set of ASCII bytes. Spans only 16 bytes.
pub type AsciiSet = AnyByteSet<CHUNKS>;
/// A compact set of all bytes. Spans only 32 bytes.
pub type ByteSet = AnyByteSet<{ 2 * CHUNKS }>;

/// A compact stack of up to 8 byte sets for fast lookup.
/// Only particular instances - `AsciiStack` and `ByteStack` can be constructed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnyByteStack<B, const N: usize> {
    masks: [u8; N],
    current: PhantomData<B>,
}

/// A compact stack of up to 8 ASCII sets for fast lookup.
pub type AsciiStack<B = ()> = AnyByteStack<B, ASCII_RANGE_LEN>;
/// A compact stack of up to 8 full byte sets for fast lookup.
pub type ByteStack<B = ()> = AnyByteStack<B, { 2 * ASCII_RANGE_LEN }>;

impl AsciiSet {
    /// Creates a new, empty, `AsciiSet`.
    pub const fn new() -> Self {
        Self { mask: [0; CHUNKS] }
    }

    /// Tests whether this set contains the `byte`.
    #[inline]
    pub const fn contains(&self, byte: u8) -> bool {
        if byte >= ASCII_RANGE_LEN as u8 {
            return false;
        };
        let chunk = self.mask[byte as usize / BITS_PER_CHUNK];
        let mask = 1 << (byte as usize % BITS_PER_CHUNK);
        (chunk & mask) != 0
    }
}

impl ByteSet {
    /// Creates a new, empty, `ByteSet`.
    pub const fn new() -> Self {
        Self {
            mask: [0; 2 * CHUNKS],
        }
    }

    /// Tests whether this set contains the `byte`.
    #[inline]
    pub const fn contains(&self, byte: u8) -> bool {
        let chunk = self.mask[byte as usize / BITS_PER_CHUNK];
        let mask = 1 << (byte as usize % BITS_PER_CHUNK);
        (chunk & mask) != 0
    }
}

impl<const N: usize> AnyByteSet<N> {
    /// Lowercase letters (`a` - `z`)
    pub const LOWERCASE: Self = Self::blank().add_range(b'a'..=b'z');
    /// Uppercase letters (`A` - `Z`)
    pub const UPPERCASE: Self = Self::blank().add_range(b'A'..=b'Z');
    /// Numerical digits (`0` - `9`)
    pub const DIGITS: Self = Self::blank().add_range(b'0'..=b'9');
    /// Uppercase and lowercase letters
    pub const ALPHABETIC: Self = Self::LOWERCASE.union(Self::UPPERCASE);
    /// Uppercase and lowercase letters and digits
    pub const ALPHANUMERIC: Self = Self::ALPHABETIC.union(Self::DIGITS);

    /// Space and tab
    pub const SPACE_TAB: Self = Self::blank().add_bytes(b" \t");
    /// Line feed and carriage return
    pub const NEWLINE: Self = Self::blank().add_bytes(b"\r\n");
    /// Space, tab, line feed and carriage return
    pub const WHITESPACE: Self = Self::SPACE_TAB.union(Self::NEWLINE);

    /// ASCII graphic characters
    pub const GRAPHIC: Self = Self::blank().add_range(b'!'..=b'~');
    /// Reserved URI characters (per RFC 3986, section 2.2)
    pub const URI_RESERVED: Self = Self::blank().add_bytes(b"!#$&'()*+,/:;=?@[]");

    const fn blank() -> Self {
        Self { mask: [0; N] }
    }

    /// Adds the `byte` to the set.
    pub const fn add(&self, byte: u8) -> Self {
        let mut mask = self.mask;
        mask[byte as usize / BITS_PER_CHUNK] |= 1 << (byte as usize % BITS_PER_CHUNK);
        Self { mask }
    }

    /// Removes the `byte` from the set.
    pub const fn remove(&self, byte: u8) -> Self {
        let mut mask = self.mask;
        mask[byte as usize / BITS_PER_CHUNK] &= !(1 << (byte as usize % BITS_PER_CHUNK));
        Self { mask }
    }

    /// Adds every byte from the slice to the set.
    pub const fn add_bytes(&self, bytes: &[u8]) -> Self {
        let mut aset = *self;
        let mut i = 0;
        while i < bytes.len() {
            aset = aset.add(bytes[i]);
            i += 1;
        }
        aset
    }

    /// Removes every byte from the slice from the set.
    pub const fn remove_bytes(&self, bytes: &[u8]) -> Self {
        let mut aset = *self;
        let mut i = 0;
        while i < bytes.len() {
            aset = aset.remove(bytes[i]);
            i += 1;
        }
        aset
    }

    /// Adds every byte from the inclusive range to the set.
    pub const fn add_range(&self, range: RangeInclusive<u8>) -> Self {
        let mut aset = *self;
        let mut c = *range.start();
        while c <= *range.end() {
            aset = aset.add(c);
            c += 1;
        }
        aset
    }

    /// Removes every byte from the inclusive range from the set.
    pub const fn remove_range(&self, range: RangeInclusive<u8>) -> Self {
        let mut aset = *self;
        let mut c = *range.start();
        while c <= *range.end() {
            aset = aset.remove(c);
            c += 1;
        }
        aset
    }

    /// Returns the union of this set and `other`.
    /// 
    /// #Panics
    /// Panics if the size of `other` is bigger than the size of `self`.
    ///
    /// # Examples
    /// ```
    /// use bset::AsciiSet;
    /// assert_eq!(AsciiSet::ALPHABETIC, AsciiSet::UPPERCASE.union(AsciiSet::LOWERCASE));
    /// ```
    pub const fn union<const M: usize>(&self, other: AnyByteSet<M>) -> Self {
        let mut mask = [0; N];
        let mut i = 0;
        while i < N {
            mask[i] = self.mask[i] | other.mask[i];
            i += 1;
        }
        Self { mask }
    }

    /// Returns the intersection of this set and `other`.
    /// 
    /// #Panics
    /// Panics if the size of `other` is bigger than the size of `self`.
    ///
    /// # Examples
    /// ```
    /// use bset::AsciiSet;
    /// assert_eq!(AsciiSet::LOWERCASE, AsciiSet::ALPHABETIC.intersection(AsciiSet::LOWERCASE));
    /// ```
    pub const fn intersection<const M: usize>(&self, other: AnyByteSet<M>) -> Self {
        let mut mask = [0; N];
        let mut i = 0;
        while i < N {
            mask[i] = self.mask[i] & other.mask[i];
            i += 1;
        }
        Self { mask }
    }

    /// Returns the set of all ASCII chars not in `self`.
    pub const fn complement(&self) -> Self {
        let mut mask = self.mask;
        let mut i = 0;
        while i < N {
            mask[i] = !mask[i];
            i += 1;
        }
        Self { mask }
    }

    /// Returns the set of chars in `self` but not `other`.
    /// 
    /// #Panics
    /// Panics if the size of `other` is bigger than the size of `self`.
    ///
    /// # Examples
    /// ```
    /// use bset::AsciiSet;
    /// assert_eq!(AsciiSet::LOWERCASE, AsciiSet::ALPHABETIC.difference(AsciiSet::UPPERCASE));
    /// ```
    pub const fn difference<const M: usize>(&self, other: AnyByteSet<M>) -> Self {
        self.intersection(other.complement())
    }
}

impl<T> AsciiStack<T> {
    /// Tests whether the set at the position `B` in the stack contains the `byte`.
    #[inline]
    pub fn contains<B: Bit>(&self, byte: u8) -> bool {
        byte < ASCII_RANGE_LEN as u8 && self.masks[byte as usize] & (1 << B::NUMBER) != 0
    }
}

impl AsciiStack<B0> {
    /// Creates a new `AsciiStack`
    pub const fn new() -> Self {
        Self {
            masks: [0; ASCII_RANGE_LEN],
            current: PhantomData,
        }
    }
}

impl<T> ByteStack<T> {
    /// Tests whether the set at the position `B` in the stack contains the `byte`.
    #[inline]
    pub fn contains<B: Bit>(&self, byte: u8) -> bool {
        self.masks[byte as usize] & (1 << B::NUMBER) != 0
    }
}

impl ByteStack<B0> {
    /// Creates a new `ByteStack`
    pub const fn new() -> Self {
        Self {
            masks: [0; 2 * ASCII_RANGE_LEN],
            current: PhantomData,
        }
    }
}

// TODO: Implement this generically once generic bounds are stable for const fns.
macro_rules! implement_add_set {
    ($($ty:ty),*) => {
        $(impl<const N: usize> AnyByteStack<$ty, N> {
            /// Add this byte set to the next available position in this stack.
            pub const fn add_set<const M: usize>(
                &self,
                aset: AnyByteSet<M>,
            ) -> AnyByteStack<<$ty as Bit>::Successor, N> {
                let mut masks = self.masks;
                let mask = aset.mask;
                let mut i = 0;
                while i < M {
                    let mut j = 0;
                    while j < BITS_PER_CHUNK {
                        if mask[i] & (1 << j) != 0 {
                            masks[i * BITS_PER_CHUNK + j] |= 1 << <$ty>::NUMBER;
                        }
                        j += 1;
                    }
                    i += 1;
                }

                AnyByteStack {
                    masks,
                    current: PhantomData,
                }
            }
        })*
    }
}
implement_add_set!(B0, B1, B2, B3, B4, B5, B6, B7);
