/// The 0-th bit.
pub struct B0;
/// The 1-st bit.
pub struct B1;
/// The 2-nd bit.
pub struct B2;
/// The 3-rd bit.
pub struct B3;
/// The 4-th bit.
pub struct B4;
/// The 5-th bit.
pub struct B5;
/// The 6-th bit.
pub struct B6;
/// The 7-th bit.
pub struct B7;

pub trait Bit {
    const NUMBER: usize;
    type Successor;
}

impl Bit for B0 {
    const NUMBER: usize = 0;
    type Successor = B1;
}

impl Bit for B1 {
    const NUMBER: usize = 1;
    type Successor = B2;
}

impl Bit for B2 {
    const NUMBER: usize = 2;
    type Successor = B3;
}

impl Bit for B3 {
    const NUMBER: usize = 3;
    type Successor = B4;
}

impl Bit for B4 {
    const NUMBER: usize = 4;
    type Successor = B5;
}

impl Bit for B5 {
    const NUMBER: usize = 5;
    type Successor = B6;
}

impl Bit for B6 {
    const NUMBER: usize = 6;
    type Successor = B7;
}

impl Bit for B7 {
    const NUMBER: usize = 7;
    type Successor = ();
}
