use std::mem::transmute;

pub struct BitSlice<A> {
    addr: A, //64 bits
    len: usize, // 61 bits
    start_bit:usize, //3 bits
}

impl <'a ,ElementType: UInts> BitSlice<&'a ElementType> {


    fn to_pointer(&self) -> &'a[ElementType] {
        let mut local_len = self.len;
        local_len.set_these_bits(self.start_bit << 61, &(61..=63));
        
        // self.addr is already a reference, so we can cast it directly
        let addr_usize = self.addr as *const ElementType as usize;
        unsafe { std::mem::transmute((addr_usize, local_len)) }
    }
}

impl <'a ,ElementType: UInts> BitSlice<&'a mut ElementType> {

    fn to_mut_pointer(&mut self) -> &'a mut[ElementType] {
        let mut local_len = self.len;
        local_len.set_these_bits(self.start_bit << 61, &(61..=63));
        
        // Cast the mutable reference directly to an integer address
        let addr_usize = self.addr as *mut ElementType as usize;
        unsafe { std::mem::transmute((addr_usize, local_len)) }
    }
    }

pub trait BitSliceOps<ElementType> {
    // Read-only variant takes &self
    fn to_bitslice(&self) -> BitSlice<&ElementType>;

    // Writable variant takes &mut self
    fn to_bitslice_mut(&mut self) -> BitSlice<&mut ElementType>;
}

impl<ElementType: UInts> BitSliceOps<ElementType> for [ElementType] {
    fn to_bitslice(&self) -> BitSlice<&ElementType> {
        let (addr, local_len): (usize, usize) = unsafe { std::mem::transmute(self) };
        let local = local_len.get_bits(&(61..=63));
        let len = local_len.get_bits(&(0..61));
        BitSlice {addr: unsafe { &*(addr as *const ElementType) },len,start_bit: local}
    }

    fn to_bitslice_mut(&mut self) -> BitSlice<&mut ElementType> {
        let (addr, local_len): (usize, usize) = unsafe { std::mem::transmute(self) };
        let local = local_len.get_bits(&(61..=63));
        let len = local_len.get_bits(&(0..61));
        BitSlice {addr: unsafe { &mut *(addr as *mut ElementType) },len,start_bit: local}
    }
}
