use bitys::Bitys;
#[test]
fn get_set() {
    println!("Hello, world!");
    let mut bitys_test: Bitys<u8> = Bitys::new();
    bitys_test.set(1,true);
    bitys_test.set(2,true);
    bitys_test.set(3,true);
    assert_eq!(bitys_test.get(1),true);
    assert_eq!(bitys_test.get(2),true);
    assert_eq!(bitys_test.get(3),true);
    assert_eq!(bitys_test.get(4),false);
    println!("{},{:?}",bitys_test.get(3),bitys_test);
}

use bitys::bitops::BitOps;
#[test]
fn bitops_get_set() {
    let mut num:u8 = 8; //0001
    println!("{}",num.get_bit(3));
    assert_eq!(num.get_bit(0), false);
    assert_eq!(num.get_bit(1), false);
    assert_eq!(num.get_bit(2), false);
    assert_eq!(num.get_bit(3), true);

    num.set_bit(7, true);
    println!("{}",num.get_bit(7));
    assert_eq!(num, (8+2_u8.pow(7))); 
    assert_eq!(num.get_bit(7), true);
    assert_eq!(num.get_bit(3), true);
    println!("{}",num);
}

#[test]
fn bitops_bitmask() {
    let num:u8 =BitOps::bitmask(&(0..8));
    println!("bitmask:{:?}",num);
    assert_eq!(num as usize, 2_usize.pow(8)-1)
}

#[test]
fn bitops_popcnt_ctz() {
    let num:u8 = u8::MAX;
    println!("{}", num.ctz(&(0..8)));
    assert_eq!(num.count_zeros() as usize,num.ctz(&(0..8)));
    assert_eq!(num.count_ones() as usize,num.popcnt(&(0..8)));

    let num:u8 = 2_u8.pow(7)-1;
    assert_eq!(num.count_zeros() as usize,num.ctz(&(0..=7)));
    assert_eq!(num.count_ones() as usize,num.popcnt(&(0..=7)));
}

#[test]
fn bitops_get_set_bits(){
    let mut num: u8 = 0;
    num.set_bits(&(0..=2),true);
    println!("num: {}",num);
    assert_eq!(num, 2_u8.pow(3)-1);
    assert_eq!(num, num.get_bits(&(0..=2)));
}

#[test]
fn bitops_first_last_set_bit() {
    let bit_set:u8 = 5;
    let mut num:u8 = 2_u8.pow(5); // 001
    assert_eq!(num.first_set_bit(), 5);
    assert_eq!(num.last_set_bit(), 5);
    let last_bit = 7;
    num += 2_u8.pow(last_bit);
    assert_eq!(num.last_set_bit(), 7);

}
#[test]
fn bitops_set_these_bits() {
    let mut num:u8 = 255;
    num.set_these_bits(0b0111, &(0..4));
    println!("{:08b}", num); 
    assert_eq!(num, 255-2_u8.pow(3))
}


