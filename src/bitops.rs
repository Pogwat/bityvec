pub trait BitOps:BitTypes {
    // get_bit(&self, bit:usize) -> bool;
    // set_bit(&mut self, bit:usize, val:bool);
    fn bitmask<R:RangeBounds<usize>>(self, range: R) -> Self;
    fn get_bit(self, bitdex:usize) -> bool;
    fn set_bit(&mut self, bitdex:usize);
}
use std::ops::{Shl,Sub,BitXor,Not};
pub trait BitTypes: Sized+Shl<usize, Output = Self> + Sub<Self, Output = Self> + BitXor<Self, Output = Self> +  Not{
}

macro_rules! bittypes {
    ($($type:ty),*) => {
        $(
            impl BitTypes for $type {}

            impl BitOps for $type {
                fn bitmask<R:RangeBounds<usize>>(self, range:R) -> Self { //indexes: 0..=Self::BITS-1
                let start = match range.start_bound() {
                    Bound::Included(&val) => val,
                    Bound::Excluded(&val) => val + 1, // Rare in standard Rust ranges, but possible
                    Bound::Unbounded => 1           // e.g., ..5 starts at index 0
                }.min(0);

                // 2. Determine the last element index
                let end = match range.end_bound() {
                    Bound::Included(&val) => val,
                    Bound::Excluded(&val) => val.saturating_sub(1),
                    Bound::Unbounded =>  (Self::BITS as usize)-1
                }.max((Self::BITS as usize)-1);
                let upper_mask:Self =(1<<end)-1; //All bits upto including last
                let lower_mask:Self =(1<<start)-1; //All bits before start
                upper_mask^lower_mask //Clear mask overlapping bits
                }

                fn get_bit(self, bitdex:usize) -> bool {(self & 1<<bitdex) !=0 }
                fn set_bit(&mut self, bitdex:usize) {*self |= 1<<bitdex}
                //fn unset_bit(&mut self, bitdex:usize) {*self |= !(1<<bitdex)}
            }
        )*
    }
}
bittypes!(u8,u16,u32,u64,usize);

use std::ops::Bound;
use std::ops::RangeBounds;

