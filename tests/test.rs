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


