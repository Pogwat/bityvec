use crate::UInts;
pub struct Biter<ElementType> {
    pub end_ptr:*const ElementType,
    pub end_bit:usize,
    pub ptr: *const ElementType,
    pub bit_position: usize
}

impl<ElementType: UInts + Copy> Iterator for Biter<ElementType> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let pointer_overflow = ((self.end_bit + 1) / 8) as usize;
        let final_bit_target = (self.end_bit + 1) % 8;
        unsafe { if self.ptr == self.end_ptr.add(pointer_overflow) && self.bit_position==final_bit_target {return None} }
        let bit = unsafe { (*self.ptr).get_bit(self.bit_position) };
        self.bit_position += 1;
        unsafe { self.ptr = self.ptr.add(self.bit_position>>ElementType::BITDEX_MAX); } //if bit_pos==8 travers pointer
        self.bit_position = self.bit_position & ElementType::ELEMENT_BITS-1; //reset bitpos at 8 to 0
        Some(bit)
    }
}
