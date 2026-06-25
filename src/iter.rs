use crate::UInts;
use std::marker::PhantomData;
use bit_operations::MutBitProxy;

#[macro_export]
macro_rules! define_bit_iterator {
    (
        struct_name: $name:ident,
        pointer_type: $ptr_ty:ty,
        item_type: $item_ty:ty,
        $(lifetime: $lt:tt,)? 
        bit_method: $bit_method:ident
        $(,$extra_field:ident: $extra_field_ty:ty)?
    ) => {
        pub struct $name <$($lt,)? ElementType> {
            pub end_ptr: *const ElementType,
            pub end_bit: usize,
            pub ptr: $ptr_ty,
            pub bit_position: usize,
            $(pub $extra_field: $extra_field_ty,)?
        }

        impl <$($lt,)? ElementType: UInts> Iterator for $name <$($lt,)? ElementType> {
            type Item = $item_ty;

            fn next(&mut self) -> Option<Self::Item> {
                let pointer_overflow = ((self.end_bit + 1) / 8) as usize;
                let final_bit_target = (self.end_bit + 1) % 8;
                unsafe { if self.ptr == (self.end_ptr as $ptr_ty).add(pointer_overflow) && self.bit_position == final_bit_target {return None;} }
                let bit = unsafe { (*self.ptr).$bit_method(self.bit_position) };
                self.bit_position += 1;
                unsafe { self.ptr = self.ptr.add(self.bit_position >> ElementType::BITDEX_MAX) } 
                self.bit_position = self.bit_position & (ElementType::ELEMENT_BITS - 1); 
                Some(bit)
            }

        }
        impl <$($lt,)? ElementType: UInts> $name <$($lt,)? ElementType> {
            fn new(ptr:$ptr_ty,bit_position: usize,end_ptr: $ptr_ty,end_bit: usize ) -> Self {
                Self { ptr,bit_position,end_ptr,end_bit,$( $extra_field: <$extra_field_ty>::default(),)?} 
            }
        }
    };
}

define_bit_iterator!(
    struct_name: Biter,
    pointer_type: *const ElementType,
    item_type: bool,
    bit_method: get_bit
);

define_bit_iterator!(
    struct_name: BiterMut,
    pointer_type: *mut ElementType,
    item_type: MutBitProxy<'a, ElementType>,
    lifetime: 'a,
    bit_method: mut_bit,
    _marker: PhantomData<&'a mut ElementType>
);