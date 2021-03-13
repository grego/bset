#![feature(test)]
extern crate test;

use bset::{bits::*, AsciiSet, AsciiStack, ByteSet, ByteStack};
use rand::{thread_rng, Rng};
use test::Bencher;

use byte_set::ByteSet as OtherByteSet;

const SAMPLE_SIZE: usize = 1024;

const ASCII_STACK: AsciiStack<B4> = AsciiStack::new()
    .add_set(AsciiSet::LOWERCASE)
    .add_set(AsciiSet::ALPHABETIC)
    .add_set(AsciiSet::ALPHANUMERIC)
    .add_set(AsciiSet::URI_RESERVED);

const BYTE_STACK: ByteStack<B4> = ByteStack::new()
    .add_set(AsciiSet::LOWERCASE)
    .add_set(AsciiSet::ALPHABETIC)
    .add_set(AsciiSet::ALPHANUMERIC)
    .add_set(AsciiSet::URI_RESERVED);

type Lowercase = B0;
type Alphabetic = B1;
type Alphanumeric = B2;
type UriReserved = B3;

fn bench_fn<F>(b: &mut Bencher, f: F)
where
    F: Fn(&u8) -> bool,
{
    let mut input = [0_u8; SAMPLE_SIZE];
    thread_rng().fill(&mut input);
    b.bytes = SAMPLE_SIZE as u64;
    b.iter(|| input.iter().copied().filter(&f).count());
}

// Lowercase cases
#[bench]
fn lowercase_ascii_set(b: &mut Bencher) {
    bench_fn(b, |&c| AsciiSet::LOWERCASE.contains(c));
}

#[bench]
fn lowercase_ascii_stack(b: &mut Bencher) {
    bench_fn(b, |&c| ASCII_STACK.contains::<Lowercase>(c));
}

#[bench]
fn lowercase_byte_set(b: &mut Bencher) {
    bench_fn(b, |&c| ByteSet::LOWERCASE.contains(c));
}

#[bench]
fn lowercase_byte_stack(b: &mut Bencher) {
    bench_fn(b, |&c| BYTE_STACK.contains::<Lowercase>(c));
}

#[bench]
fn lowercase_match(b: &mut Bencher) {
    bench_fn(b, |&c| matches!(c, b'a'..=b'z'));
}

#[bench]
fn lowercase_is_lowercase(b: &mut Bencher) {
    bench_fn(b, |&c| c.is_ascii_lowercase());
}

#[bench]
fn lowercase_other_byte_set(b: &mut Bencher) {
    bench_fn(b, |&c| OtherByteSet::ASCII_LOWERCASE.contains(c));
}

// Alphabetic cases
#[bench]
fn letter_ascii_set(b: &mut Bencher) {
    bench_fn(b, |&c| AsciiSet::ALPHABETIC.contains(c));
}

#[bench]
fn letter_ascii_stack(b: &mut Bencher) {
    bench_fn(b, |&c| ASCII_STACK.contains::<Alphabetic>(c));
}

#[bench]
fn letter_byte_set(b: &mut Bencher) {
    bench_fn(b, |&c| ByteSet::ALPHABETIC.contains(c));
}

#[bench]
fn letter_byte_stack(b: &mut Bencher) {
    bench_fn(b, |&c| BYTE_STACK.contains::<Alphabetic>(c));
}

#[bench]
fn letter_is_alphabetic(b: &mut Bencher) {
    bench_fn(b, |&c| c.is_ascii_alphabetic());
}

#[bench]
fn letter_match(b: &mut Bencher) {
    bench_fn(b, |&c| matches!(c, b'a'..=b'z' | b'A'..=b'Z'));
}

#[bench]
fn letter_other_byte_set(b: &mut Bencher) {
    bench_fn(b, |&c| OtherByteSet::ASCII_ALPHABETIC.contains(c));
}

// Alphanumeric cases
#[bench]
fn alnum_ascii_set(b: &mut Bencher) {
    bench_fn(b, |&c| AsciiSet::ALPHANUMERIC.contains(c));
}

#[bench]
fn alnum_ascii_stack(b: &mut Bencher) {
    bench_fn(b, |&c| ASCII_STACK.contains::<Alphanumeric>(c));
}

#[bench]
fn alnum_byte_set(b: &mut Bencher) {
    bench_fn(b, |&c| ByteSet::ALPHANUMERIC.contains(c));
}

#[bench]
fn alnum_byte_stack(b: &mut Bencher) {
    bench_fn(b, |&c| BYTE_STACK.contains::<Alphanumeric>(c));
}

#[bench]
fn alnum_is_alnum(b: &mut Bencher) {
    bench_fn(b, |&c| c.is_ascii_alphanumeric());
}

#[bench]
fn alnum_match(b: &mut Bencher) {
    bench_fn(b, |&c| matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9'));
}

#[bench]
fn alnum_other_byte_set(b: &mut Bencher) {
    bench_fn(b, |&c| OtherByteSet::ASCII_ALPHANUMERIC.contains(c));
}

// URI reserved cases
#[bench]
fn uri_ascii_set(b: &mut Bencher) {
    bench_fn(b, |&c| AsciiSet::URI_RESERVED.contains(c));
}

#[bench]
fn uri_ascii_stack(b: &mut Bencher) {
    bench_fn(b, |&c| ASCII_STACK.contains::<UriReserved>(c));
}

#[bench]
fn uri_byte_set(b: &mut Bencher) {
    bench_fn(b, |&c| ByteSet::URI_RESERVED.contains(c));
}

#[bench]
fn uri_byte_stack(b: &mut Bencher) {
    bench_fn(b, |&c| BYTE_STACK.contains::<UriReserved>(c));
}

#[bench]
fn uri_match(b: &mut Bencher) {
    bench_fn(b, |&c| {
        matches!(
            c,
            b'!' | b'#'
                | b'$'
                | b'&'
                | b'\''
                | b'('
                | b')'
                | b'*'
                | b'+'
                | b','
                | b'/'
                | b':'
                | b';'
                | b'='
                | b'?'
                | b'@'
                | b'['
                | b']'
        )
    });
}
