//self[index] == *(self[index]) not (*self)[index] , deref on index return not self
use crate::UInts;

pub struct MutBitProxy<'a,ElementType: UInts> {
    val:bool,
    addr: &'a mut ElementType,
    bit:u8    
}

use std::ops::Deref;

impl<'a,ElementType: UInts> Deref for MutBitProxy<'a,ElementType> {
    type Target = bool;
    fn deref(&self) -> &Self::Target {&self.val} //Cant mutate cuz &self
}

use std::ops::DerefMut;

impl <'a,ElementType: UInts> DerefMut for MutBitProxy<'a,ElementType> {
    fn deref_mut(&mut self) -> &mut Self::Target {&mut self.val}
}

impl<'a, ElementType: UInts> Drop for MutBitProxy<'a, ElementType> {
    fn drop(&mut self) {self.addr.set_bit(self.bit as usize, self.val)}
}

impl <'a,ElementType: UInts> MutBitProxy<'a,ElementType> {
    fn new(addr:&'a mut ElementType,bit:usize) -> Self {
        Self {val: addr.get_bit(bit),addr,bit:bit as u8}
    }
}