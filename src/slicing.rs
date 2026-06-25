use std::mem::transmute;
use std::marker::PhantomData;
use bit_operations::BitOps;
use crate::UInts;

pub struct Mutable;
pub struct Immutable;
/* START, START BIT, END, END_BIT */
pub struct BitSlice<Mutability> {
    end_bit:usize, //64bits
    start_bit:usize, //64bits
    _mutability: PhantomData<Mutability>, // 0 bits
}

impl <Mutability> BitSlice<Mutability> {
    pub fn to_pointer<'a, ElementType: UInts>(&self) -> &'a [ElementType] {unsafe { core::mem::transmute((self.start_bit,self.end_bit)) }}
    pub fn to_mut_pointer<'a, ElementType: UInts>(&mut self) -> &'a mut [ElementType] {unsafe { std::mem::transmute((self.start_bit,self.end_bit)) }}
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
                    start_bit: start,
                    end_bit: end,
                    _mutability: PhantomData, 
                }
            }
        }
    };
}

slice_mutability!((ElementType: UInts), [ElementType], Immutable);
slice_mutability!((ElementType: UInts), &mut [ElementType], Mutable);
slice_mutability!((ElementType: UInts), & [ElementType], Immutable);