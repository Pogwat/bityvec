/* Goals:
SIMD lookup
Slices
ctz
popcnt
Indexing
set/get \,
*/

use bit_field::BitField;

pub trait UInts: BitField + Default + Clone + Copy {const ELEMENT_BITS:usize; const ELEMENT_INADDR_BITS: usize;const LEN_BITS:usize;}
macro_rules! unints {($($type:ty),*) => {
    $(impl UInts for $type { //[ADDR, LEN] -> [ADDR, BITADDR,LEN]
        const ELEMENT_BITS:usize=std::mem::size_of::<$type>() * 8;
        const ELEMENT_INADDR_BITS: usize = Self::ELEMENT_BITS.ilog2() as usize;  //Bits for addresing any bit in a element
        const LEN_BITS:usize = (std::mem::size_of::<usize>()*8)- Self::ELEMENT_INADDR_BITS; //max slice bits
    })*} //[StartElem(64), StartBit(6), Len(58)]
}
unints!(u8,u16,u32,u64);

#[derive(Debug)]
pub struct Bitys<ElementType: UInts> {pub bytes:Vec<ElementType>}

impl<ElementType: UInts> Bitys<ElementType> {
    pub fn bit_idx(bitdex:usize) -> usize {bitdex%ElementType::ELEMENT_BITS}
    pub fn elems_idx(bitdex:usize) -> usize {bitdex/ElementType::ELEMENT_BITS}
    pub fn get(&self, index:usize) -> bool {self.bytes[Self::elems_idx(index)].get_bit(Self::bit_idx(index))}
    pub fn new_empty_elem(&mut self) {self.bytes.push(ElementType::default())}
    pub fn new_empty_elems(&mut self, empty_elems:usize) {self.bytes.resize(self.bytes.len()+empty_elems,ElementType::default())}
    pub fn set(&mut self, bitdex:usize, val:bool) {
        let elems_idx = Self::elems_idx(bitdex);
        if elems_idx+1>self.bytes.len() {self.new_empty_elems(elems_idx+1-self.bytes.len())} //index+1>self.len() == index>self.len()-1, algebra add 1 to both sides, thsi repvents underflow if self.len()==0, as 0-1 = -1 usize cant be negative
        self.bytes[elems_idx].set_bit(Self::bit_idx(bitdex),val);
    }
    pub fn new() -> Self{ Self{bytes:Vec::new()}  }
} //Excluding genercis I have a full working bitvec in lines:25-39 just 14 lines of code!

// struct BitProxy<'a,ElementType: UInts> {
//     byte: &'a mut ElementType,
//     bit: usize, // 0 to 7
// }