use std::mem::transmute;
use std::marker::PhantomData;
use bit_operations::BitOps;
use crate::UInts;

pub struct Mutable;
pub struct Immutable;
/* START, START BIT, END, END_BIT */
pub struct BitSlice<Mutability> {
    start_element:usize, //61 bits
    start_bit:usize, //3 bits
    end_element:usize, //61 bits
    end_bit: usize, // 3 bits
    _mutability: PhantomData<Mutability>, // 0 bits
}

impl <Mutability> BitSlice<Mutability> {
    fn wrap(&self) -> (usize,usize) {
        let mut start = self.start_element;
        start.set_these_bits(self.start_bit << 61, &(61..=63));
        let mut end = self.end_element;
        end.set_these_bits(self.end_bit << 61, &(61..=63));
        (start,end)
    }

    pub fn to_pointer<'a, ElementType: UInts>(&self) -> &'a [ElementType] {unsafe { std::mem::transmute(self.wrap()) }}
    pub fn to_mut_pointer<'a, ElementType: UInts>(&mut self) -> &'a mut [ElementType] {unsafe { std::mem::transmute(self.wrap()) }}
        
}

pub trait MutabilityFlag {type MutFlag;}

pub trait BitSliceOps<Mutability>: MutabilityFlag {
    fn to_bitslice(&self) -> BitSlice<Mutability>;
}

macro_rules! slice_mutability {
    (($($generics:tt)*), $target_type:ty, $mutability:ty) => {
        impl<$($generics)*> MutabilityFlag for $target_type {type MutFlag = $mutability;}

        impl<$($generics)*> BitSliceOps<$mutability> for $target_type {
            fn to_bitslice(&self) -> BitSlice<$mutability> {
                let (start, end): (usize, usize) = unsafe {  *(self as *const Self as *const (usize, usize)) }; //Pointer to (self as 2 usize pointers) then deref
                BitSlice {
                    start_element: start.get_bits(&(0..61)), 
                    start_bit: start.get_bits(&(61..=63)) >> 61, 
                    end_element: end.get_bits(&(0..61)), 
                    end_bit: end.get_bits(&(61..=63)) >> 61, 
                    _mutability: PhantomData, 
                }
            }
        }
    };
}

slice_mutability!((ElementType: UInts), [ElementType], Immutable);
slice_mutability!((ElementType: UInts), &mut [ElementType], Mutable);
slice_mutability!((ElementType: UInts), & [ElementType], Immutable);