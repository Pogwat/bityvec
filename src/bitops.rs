pub trait BitOps:BitTypes {
    // get_bit(&self, bit:usize) -> bool;
    // set_bit(&mut self, bit:usize, val:bool);
    fn bitmask<R:RangeBounds<usize>+ NumRangeExtract<usize>>(range: &R) -> Self;
    fn get_bit(&self, bitdex:usize) -> bool;
    fn set_bit(&mut self, bitdex:usize, val:bool);
    fn get_bits<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> Self;
    fn ctz<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> usize;
    fn popcnt<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> usize;
}
use std::ops::{Shl,Sub,BitXor,Not};
pub trait BitTypes: Sized+Shl<usize, Output = Self> + Sub<Self, Output = Self> + BitXor<Self, Output = Self> +  Not{}

macro_rules! bittypes {
    ($($type:ty),*) => {
        $(
            impl BitTypes for $type {}

            impl BitOps for $type {
                fn bitmask<R:RangeBounds<usize>+ NumRangeExtract<usize>>(range:&R) -> Self { //indexes: 0..=Self::BITS-1
                    let start = range.start().unwrap_or(0).max(0);
                    let end = range.end().unwrap_or(Self::BITS as usize).min((Self::BITS as usize)-1);
                    if end>=(Self::BITS - 1) as usize {return Self::MAX } //cant shift 1 by Self::BITS, this overflows
                    let upper_mask:Self =(1<<end+1)-1; //All bits upto including last
                    let lower_mask:Self =(1<<start)-1; //All bits before start
                    upper_mask^lower_mask //Clear mask overlapping bits
                }

                fn get_bit(&self, bitdex:usize) -> bool {(self & 1<<bitdex) !=0 }
                fn set_bit(&mut self, bitdex:usize, val:bool) {*self |= (val as $type)<<bitdex}
                fn ctz<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> usize {
                    ((!Self::bitmask(range)) | self).count_zeros() as usize //  111111BitsWeWant1111111 , others are 1
                }
                fn get_bits<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> Self {Self::bitmask(range) & self}
                fn popcnt<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> usize {
                    self.get_bits(range).count_ones() as usize
                }

            }
        )*
    }
}
bittypes!(u8,u16,u32,u64,usize);

use std::ops::Bound;
use std::ops::RangeBounds;

pub trait NumRangeExtract<T>: RangeBounds<T>  {
    fn end(&self) -> Option<T>;
    fn start(&self) -> Option<T>;
}

macro_rules! num_rangy {
    ($($type:ty),*) => {
        $(
            impl <R:RangeBounds<$type>>NumRangeExtract<$type> for R {
                fn end(&self) -> Option<$type> {
                    match self.end_bound() {
                        Bound::Included(val) => Some(*val),
                        Bound::Excluded(val) => Some((*val).saturating_sub(1)),
                        Bound::Unbounded =>  None
                    }
                }
                fn start(&self) -> Option<$type> {
                    match self.start_bound() {
                        Bound::Included(val) => Some(*val),
                        Bound::Excluded(val) => Some((*val).saturating_add(1)), // Rare in standard Rust ranges, but possible
                        Bound::Unbounded => None           // e.g., ..5 starts at index 0
                    }
                }
            }
        )*
    }
}

num_rangy!(u8,u16,u32,u64,usize);