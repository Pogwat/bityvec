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

#[test]
fn iter() {
    let mut bitys_test: Bitys<u8> = Bitys::new();
    bitys_test.set(10,true);
    bitys_test.set(0,true);
    bitys_test.set(15,true);
    println!("last: {}",bitys_test.get(15));
    let mut len=0;
    assert_eq!(bitys_test.get(10),true);
        bitys_test.iter().enumerate().for_each(|(idx,bit)| {
        if idx == 10 {assert_eq!(bit,true)}
        if idx == 15 {assert_eq!(bit,true)}
        if idx == 0 {assert_eq!(bit,true)}
        if idx == 5 {assert_eq!(bit,false)}
        len+=1;
        println!("{}",bit)
    });
    assert_eq!(len, bitys_test.bytes.len()*(u8::BITS as usize))
} 


