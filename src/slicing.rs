use std::marker::PhantomData;
use crate::UInts;
use crate::iter::*;

pub struct BitSlice<'a,Mutability:MPtr<ElementType>, ElementType> {
        pub start_ptr: Mutability::PointerType,
        pub start_bit: usize, 
        pub end_bit: usize, 
        pub relative_end_element: usize, 
        pub _element_type: PhantomData<&'a ElementType>,
}

pub trait MPtr<ElementType> {type PointerType;}
pub struct Immutable;
pub struct Mutable;
impl <ElementType> MPtr<ElementType> for Mutable {type PointerType = *mut ElementType;}
impl <ElementType> MPtr<ElementType> for Immutable {type PointerType = *const ElementType;}

impl <'a,ElementType:UInts> BitSlice<'a,Mutable, ElementType> {
    pub fn iter_mut(&mut self) -> BiterMut<'_,ElementType> { 
        BiterMut {
            end_ptr: unsafe { self.start_ptr.add(self.relative_end_element) }, 
            end_bit: self.end_bit,
            ptr: self.start_ptr,
            bit_position: self.start_bit,
            _marker: PhantomData
        }
    }
}
use std::ops::Index;
macro_rules! bit_slices { //Shared Methods 
    ($($mutability:ty),*) => {
        $(
            impl <'a,ElementType:UInts> BitSlice<'a,$mutability,ElementType> { 
                pub fn iter(&self) -> Biter<ElementType> {
                    Biter {
                        end_ptr: unsafe { self.start_ptr.add(self.relative_end_element) }, 
                        end_bit: self.end_bit,
                        ptr: self.start_ptr,
                        bit_position: self.start_bit
                    }
                }
            }

            impl <'a, ElementType:UInts> Index<usize> for BitSlice<'a,$mutability,ElementType> {
                type Output = bool;
                fn index(&self, index: usize) -> &Self::Output { //MUST RETURN REF TO SELF
                    unsafe { if (*self.start_ptr.add(index/ElementType::ELEMENT_BITS)).get_bit(index%ElementType::ELEMENT_BITS) {&true} else {&false} }
                }
            }
        )*
    }
}
bit_slices!(Immutable, Mutable);