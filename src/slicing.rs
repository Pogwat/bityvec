use std::mem::transmute;
use bit_operations::BitOps;
use crate::UInts;
/* END_BIT,START_BIT,END_ELEM,ADDR */
pub struct BitSlice<A> {
    addr: A, //64 bits
    end_bit: usize, // 3 bits
    start_bit:usize, //3 bits
    end_element:usize //58 bits
}

impl<A> BitSlice<A> {
    fn wrap_meta(&self) -> usize {
        let mut end_bit_start_bit_end_element = self.end_element;
        end_bit_start_bit_end_element.set_these_bits(self.start_bit << 58, &(58..61));
        end_bit_start_bit_end_element.set_these_bits(self.end_bit << 61, &(61..=63));
        end_bit_start_bit_end_element
    }

    pub fn to_pointer<'a, ElementType: UInts>(&self) -> &'a [ElementType] 
    where A: AsRef<ElementType> /*Cant redeclare Generics*/ {
        let addr = self.addr.as_ref() as *const ElementType as usize;
        unsafe { std::mem::transmute((addr, self.wrap_meta())) }
    }

    pub fn to_mut_pointer<'a, ElementType: UInts>(&mut self) -> &'a mut [ElementType] 
    where A: AsMut<ElementType>, /*Cant redeclare Generics*/ {
        let addr = self.addr.as_mut() as *mut ElementType as usize;
        unsafe { std::mem::transmute((addr, self.wrap_meta())) }
    }
}

pub trait BitSliceOps<ElementType> {
    fn to_bitslice(&self) -> BitSlice<&ElementType>;
    fn to_bitslice_mut(&mut self) -> BitSlice<&mut ElementType>;
    fn unwrap_meta(meta:usize) -> (usize,usize,usize); //(end_bit,start_bit,end_element)
}

impl<ElementType: UInts> BitSliceOps<ElementType> for [ElementType] {
    fn unwrap_meta(meta:usize) -> (usize,usize,usize) {( meta.get_bits(&(61..=63)), meta.get_bits(&(58..61)),meta.get_bits(&(0..58)) ) }

    fn to_bitslice(&self) -> BitSlice<&ElementType> {
        let (addr,end_element_start_bit_end_bit): (usize, usize) = unsafe { std::mem::transmute(self) };
        let (end_bit,start_bit,end_element) = Self::unwrap_meta(end_element_start_bit_end_bit);
        BitSlice {addr: unsafe { &*(addr as *const ElementType) },end_element,start_bit,end_bit}
    }

    fn to_bitslice_mut(&mut self) -> BitSlice<&mut ElementType> {
        let (addr,end_element_start_bit_end_bit): (usize, usize) = unsafe { std::mem::transmute(self) };
        let (end_bit,start_bit,end_element) = Self::unwrap_meta(end_element_start_bit_end_bit);
        BitSlice {addr: unsafe { &mut *(addr as *mut ElementType) },end_element,start_bit,end_bit}
    }
}