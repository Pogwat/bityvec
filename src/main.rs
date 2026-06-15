/* Goals:
SIMD lookup
Slices
ctz
popcnt
Indexing
set/get \,
*/

fn main() {
    println!("Hello, world!");
    let mut bitys_test: Bitys<u8> = Bitys::new();
    bitys_test.set(5,true);
    println!("{}",bitys_test.get(5));
}

use bit_field::BitField;

pub trait UInts {const ELEMENT_BITS:usize;}
macro_rules! unints {($($type:ty),*) => {
    $(impl UInts for $type {const ELEMENT_BITS:usize=std::mem::size_of::<$type>() * 8;})*}
}
unints!(u8,u16,u32,u64);

struct Bitys<ElementType: UInts> {
    pub bytes:Vec<ElementType>,
    pub set_bits:usize
}

impl<ElementType: UInts + BitField + Default + Clone + Copy> Bitys<ElementType> {
    pub fn bit_idx(bitdex:usize) -> usize {bitdex%ElementType::ELEMENT_BITS}
    pub fn elems_idx(bitdex:usize) -> usize {bitdex/ElementType::ELEMENT_BITS}
    pub fn missalignment(&self) -> usize {Self::bit_idx(self.set_bits)}
    pub fn get(&self, index:usize) -> bool {self.bytes[Self::elems_idx(index)].get_bit(Self::bit_idx(index))}
    pub fn new_empty_elem(&mut self) {self.bytes.push(ElementType::default())}
    pub fn new_empty_elems(&mut self, empty_elems:usize) {self.bytes.resize(self.bytes.len()+empty_elems,ElementType::default())}
    pub fn set(&mut self, bitdex:usize, val:bool) {
        let elems_idx = Self::elems_idx(bitdex);
        if elems_idx+1>self.bytes.len() {self.new_empty_elems(elems_idx+1-self.bytes.len())} //index+1>self.len() == index>self.len()-1, algebra add 1 to both sides, thsi repvents underflow if self.len()==0, as 0-1 = -1 usize cant be negative
        self.bytes[elems_idx].set_bit(Self::bit_idx(bitdex),val);
    }
    pub fn new() -> Self{ Self{bytes:Vec::new(), set_bits:0}  }
} //Excluding genercis I have a full working bitvec in lines:17-35 just 18 lines of code!