/* Goals:
SIMD lookup
Slices
ctz
popcnt
Index \,
IndexMut
set/get \,
*/
use bit_operations::BitOps;
use bit_operations::MutBitProxy;
mod slicing;
use slicing::*;
mod iter;
use iter::*;

pub trait UInts: BitOps + Default + Clone + Copy {
    const ELEMENT_BITS:usize; 
    const ELEMENT_INADDR_BITS: usize;
    const LEN_BITS:usize;
    const BITDEX_MAX:usize;
}
macro_rules! unints {($($type:ty),*) => {
    $(impl UInts for $type { //[ADDR, LEN] -> [ADDR, BITADDR,LEN]
        const ELEMENT_BITS:usize=Self::BITS as usize;
        const ELEMENT_INADDR_BITS: usize = Self::ELEMENT_BITS.ilog2() as usize;  //Bits for addresing any bit in a element
        const LEN_BITS:usize = (std::mem::size_of::<usize>()*8)- Self::ELEMENT_INADDR_BITS; //max slice bits
        const BITDEX_MAX:usize = Self::ELEMENT_BITS.ilog2() as usize;
    })*} //[StartElem(64), StartBit(6), Len(58)]
}
unints!(u8,u16,u32,u64);

#[derive(Debug)]
pub struct Bitys<ElementType: UInts> {pub bytes:Vec<ElementType>}

impl<ElementType: UInts> Bitys<ElementType> {
    const TRUE:bool = true; //Index trait must return refrence to self
    const FALSE:bool = false; //Index trait must return refrence to self
    pub fn bit_idx(bitdex:usize) -> usize {bitdex%ElementType::ELEMENT_BITS}
    pub fn type_idx(bitdex:usize) -> usize {bitdex/ElementType::ELEMENT_BITS}
    pub fn get(&self, index:usize) -> bool {self.bytes[Self::type_idx(index)].get_bit(Self::bit_idx(index))}
    pub fn add_empty(&mut self) {self.bytes.push(ElementType::default())}
    pub fn add_emptys(&mut self, empty_elems:usize) {self.bytes.resize(self.bytes.len()+empty_elems,ElementType::default())}
    pub fn set(&mut self, bitdex:usize, val:bool) {
        let type_idx = Self::type_idx(bitdex);
        if type_idx+1>self.bytes.len() {self.add_emptys(type_idx+1-self.bytes.len())} //index+1>self.len() == index>self.len()-1, algebra add 1 to both sides, thsi repvents underflow if self.len()==0, as 0-1 = -1 usize cant be negative
        self.bytes[type_idx].set_bit(Self::bit_idx(bitdex),val);
    }
    pub fn new() -> Self{ Self{bytes:Vec::new()}  }
    pub fn get_mut(&mut self, bitdex:usize) -> MutBitProxy<'_,ElementType> {self.bytes[Self::type_idx(bitdex)].mut_bit(Self::bit_idx(bitdex))}
    pub fn iter(&self) -> Biter<ElementType> {Biter {end_ptr:(&self.bytes[self.bytes.len()-1] as *const ElementType), end_bit:7,ptr: &self.bytes[0] as *const ElementType,bit_position: 0}
}
} //Excluding genercis I have a full working bitvec in lines:25-39 just 14 lines of code!

use std::ops::Index;
impl <'a, ElementType:UInts> Index<usize> for Bitys<ElementType> {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output { //MUST RETURN REF TO SELF
        if self.get(index) {&Self::TRUE} else {&Self::FALSE}
    }
}