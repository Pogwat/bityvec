use bitys::Bitys;
#[test]
fn get_set() {
    println!("Hello, world!");
    let mut bitys_test: Bitys<u8> = Bitys::new();
    bitys_test.push(true);
    bitys_test.push(true);
    bitys_test.push(true);
    assert_eq!(bitys_test.get(0),true);
    assert_eq!(bitys_test.get(1),true);
    assert_eq!(bitys_test.get(2),true);
    bitys_test.set(1,false);
    assert_eq!(bitys_test.get(1),false);
    //println!("{},{:?}",bitys_test.get(3),bitys_test);
}

#[test]
fn iter() {
    let mut bitys_test: Bitys<u8> = Bitys::new();
    bitys_test.push(true);
    bitys_test.push(true);
    bitys_test.push(false);
    println!("last: {}",bitys_test.get(2));
    let mut len=0;
    assert_eq!(bitys_test.get(0),true);
        bitys_test.iter().enumerate().for_each(|(idx,bit)| {
        if idx == 1 {assert_eq!(bit,true)}
        if idx == 0 {assert_eq!(bit,true); println!("Zeroth element: {} ",bit)}
        if idx == 2 {assert_eq!(bit,false)}
        if idx == 3 {panic!("Indexing: {} when len is {}",len,bitys_test.len)}
        if idx == 4 {panic!("Indexing: {} when len is {}",len,bitys_test.len)}
        len+=1;
        println!("{}",bit)
    });
    assert_eq!(len, 3);
    assert_eq!(bitys_test.len, 3);
} 

#[test]
fn iter_mut() {
    let mut bitys_test: Bitys<u8> = Bitys::new();
    bitys_test.push(true);
    bitys_test.push(true);
    bitys_test.push(false);
    println!("last: {}",bitys_test.get(2));
    let mut len=0;
    assert_eq!(bitys_test.get(0),true);
        bitys_test.iter_mut().enumerate().for_each(|(idx,mut bit)| {
        println!("{}th element: {:?}",idx,*bit);
        if idx == 0 {assert_eq!(*bit,true)}
        if idx == 1 {assert_eq!(*bit,true)}
        if idx == 2 {assert_eq!(*bit,false)}
        {*bit = false;}
        len+=1;
    });
    assert_eq!(bitys_test.get(0),false);
    assert_eq!(bitys_test.get(1),false);
    assert_eq!(bitys_test.get(2),false);
    assert_eq!(len, 3);
    assert_eq!(bitys_test.len, 3);
} 

#[test]
fn slicings() { 
    let mut bitys: Bitys<u8> = Bitys::new();
    bitys.push(true);
    bitys.push(true);
    bitys.push(false);
    let zf = bitys.slice(&(0..=2));
    zf.iter().enumerate().for_each(|(bitdex,bit)| println!("{}:{}",bitdex,bit));
    assert_eq!(zf[0],true);  
    assert_eq!(zf[1],true); 
    assert_eq!(zf[2],false);
}