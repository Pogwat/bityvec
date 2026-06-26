/* Goals:
SIMD lookup
Slices
ctz
popcnt
Index \,
IndexMut
set/get \,
iter(mut) \,
*/
use bit_operations::BitOps;
use bit_operations::MutBitProxy;
use bit_operations::NumRangeExtract;
pub mod slicing;
use slicing::*;
mod iter;
use iter::*;
use std::marker::PhantomData;

pub trait UInts: BitOps + Default + Copy {
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
unints!(u8,u16,u32,u64,u128,i8,i16,i32,i64,i128,usize);

#[derive(Debug)]
pub struct Bitys<ElementType: UInts> {
    pub bytes:Vec<ElementType>,
    pub len:usize //num of bits set
}

impl<ElementType: UInts> Bitys<ElementType> {
    pub fn new() -> Self{ Self{bytes:Vec::new(), len:0}  }
    pub fn bit_idx(bitdex:usize) -> usize {bitdex%ElementType::ELEMENT_BITS}
    pub fn type_idx(bitdex:usize) -> usize {bitdex/ElementType::ELEMENT_BITS}
    pub fn bit_bounds(&self,bitdex:usize) {if bitdex>=self.len {panic!("Index: {} is out of bounds as it is greater than len: {}", bitdex,self.len)}}

    pub unsafe fn uncheked_get(&self, index:usize) -> bool {self.bytes[Self::type_idx(index)].get_bit(Self::bit_idx(index))}
    pub fn get(&self, index:usize) -> bool {
        self.bit_bounds(index);
        unsafe {self.uncheked_get(index)}
    }
    
    pub unsafe fn uncheked_set(&mut self, bitdex:usize, val:bool) {self.bytes[Self::type_idx(bitdex)].set_bit(Self::bit_idx(bitdex),val);}
    pub fn set(&mut self, bitdex:usize, val:bool) { 
        self.bit_bounds(bitdex);
        unsafe { self.uncheked_set(bitdex,val) };
    }
    
    pub fn push(&mut self, val:bool) {
        let type_idx = Self::type_idx(self.len);
        if type_idx>=self.bytes.len() {self.bytes.push(ElementType::default())};
        self.bytes[type_idx].set_bit(Self::bit_idx(self.len), val);
        self.len+=1;
    }
    
    pub unsafe fn uncheked_get_mut(&mut self, bitdex:usize) -> MutBitProxy<'_,ElementType> {self.bytes[Self::type_idx(bitdex)].mut_bit(Self::bit_idx(bitdex))}
    pub fn get_mut(&mut self, bitdex:usize) -> MutBitProxy<'_,ElementType> {
        self.bit_bounds(bitdex);
        unsafe {self.uncheked_get_mut(bitdex)}
    }

    pub fn iter(&self) -> Biter<ElementType> {
        Biter {
            end_ptr:&self.bytes[self.bytes.len()-1] as *const ElementType, 
            end_bit:Self::bit_idx(self.len)-1,
            ptr: &self.bytes[0] as *const ElementType,
            bit_position:0
        }
    }

    pub fn iter_mut(&mut self) -> BiterMut<'_,ElementType> {
        BiterMut {
            end_ptr: &self.bytes[self.bytes.len()-1] as *const ElementType,
            end_bit:Self::bit_idx(self.len)-1,
            ptr: &mut self.bytes[0] as *mut ElementType,
            bit_position:0,
            _marker: PhantomData
        }
    }

    pub fn slice<R:NumRangeExtract<usize>>(&self, range:&R) -> BitSlice<'_,Immutable, ElementType> {
        let start = range.start().unwrap_or(0).max(0);
        let end = range.end().unwrap_or(self.len-1).min(self.len-1);
        BitSlice {
            start_ptr: &self.bytes[Self::type_idx(start)] as *const ElementType,
            start_bit: Self::bit_idx(start), 
            end_bit: Self::bit_idx(end), 
            relative_end_element: Self::type_idx(end-start), 
            _element_type: PhantomData
        }
    }

    //pub fn slice_mut(&mut self) -> BitSlice<'a,Mutable, ElementType>;



} //Excluding genercis I have a full working bitvec in lines:25-39 just 14 lines of code!

use std::ops::Index;
impl <'a, ElementType:UInts> Index<usize> for Bitys<ElementType> {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output { //MUST RETURN REF TO SELF
        if self.get(index) {&true} else {&false}
    }
}

macro_rules! bitys {
    ($($type: ty),*) => {
        $(
            impl Bitys<$type> {
                pub fn ctz<R:NumRangeExtract<usize>>(&self, range:R) -> usize {
                    let start = range.start().unwrap_or(0).max(0);
                    let end = range.end().unwrap_or(self.len).min(self.len);
                    let start_type_idx =  Self::type_idx(start);
                    let end_type_idx =  Self::type_idx(end);
                    let end_exclude_bits = self.bytes[end_type_idx].ctz(&(Self::bit_idx(end)..));
                    let start_exclude_bits = self.bytes[start_type_idx].ctz(&(0..=Self::bit_idx(start))); 
                    let element_ctz:usize = self.bytes[start_type_idx..end_type_idx].iter().map(|block| (*block).count_zeros() as usize).sum::<usize>();
                    element_ctz - start_exclude_bits - end_exclude_bits
                }
            }
        )*
    }
}
bitys!(u8,u16,u32,u64,u128,i8,i16,i32,i64,i128,usize);