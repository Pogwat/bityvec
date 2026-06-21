//self[index] == *self[index]
use crate::UInts;
use crate::Bitys;

// impl Deref for Bitys<ElementType> 
// where ElementType: UInts {
//     type Target = BitProxy<'a, ElementType>; 
//     fn deref(&self) -> &Self::Target {&self.val}
// }

// enum Mutability<'a, ElementType> 
// where ElementType: UInts {
//     IMut(&'a ElementType),
//     Mut(&'a mut ElementType)
// }

// pub struct BitProxy<'a, ElementType> 
// where ElementType: UInts {
//     val:bool,
//     addr: Mutability<'a, ElementType>,
//     bit:u8    
// }

// impl <'a, ElementType>BitProxy<'a,ElementType>
// where ElementType: UInts + 'a {
//     fn new(addr:&'a ElementType, bit:u8) -> Self {
//         Self {val: addr.get_bit(bit as usize),addr:Mutability::<ElementType>::IMut(addr),bit}
//     }
//     fn new_mut(addr:&'a mut ElementType, bit:u8) -> Self {
//         Self {val: addr.get_bit(bit as usize),addr:Mutability::<ElementType>::Mut(addr) ,bit}
//     }
// }



// impl <'a,ElementType: UInts> Drop for MutBitProxy<'a,ElementType> {
//     fn drop(&mut self) {self.addr.set_bit(self.bit as usize, self.val)}
// }

// use std::ops::Deref;

// impl<'a,ElementType: UInts> Deref for ImutBitProxy<'a, ElementType> {
//     type Target = bool; 
//     fn deref(&self) -> &Self::Target {&self.val}
// }

// impl<'a,ElementType: UInts> Deref for MutBitProxy<'a, ElementType> {
//     type Target = bool; 
//     fn deref(&self) -> &Self::Target {&self.val}
// }

// use std::ops::DerefMut;

// impl <'a,ElementType: UInts> DerefMut for MutBitProxy<'a, ElementType> {
//     fn deref_mut(&mut self) -> &mut Self::Target {&mut self.val}
// }

// impl<'a,ElementType: UInts> Deref for Bitys<ElementType> {
//     type Target = ImutBitProxy<'a, ElementType> ; 
//     fn deref(&self) -> &Self::Target {}
// }

// impl<'a> DerefMut for BitProxy<'a> {
//     fn deref_mut(&mut self) -> &mut bool {
//         *self.val = byte.get(self.bit);
//         &mut self.val// Gives Rust a pointer to the temp slot
//     }
// }

