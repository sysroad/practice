fn main() {
    const MAGIC: u32 = 10;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = MAGIC;
    println!("The value of x is: {}", x);

    println!("The Arch size is: {}", std::mem::size_of::<isize>());

    println!("The char size is: {}", std::mem::size_of::<char>());

    println!("The u8 size is: {}", std::mem::size_of::<u8>());

    let c = 'A';
    let s = "Hello World";

    println!("The 'A' size is: {}", std::mem::size_of_val(&c));    
    println!("The String 'Hello World' size is: {}", std::mem::size_of_val(&s));

    let sb = s.as_bytes();

    println!("The 'Hello World' bytes size is: {}", sb.len());

    let s2 = "헬로 월드";

    println!("The String '헬로 월드' size is: {}", std::mem::size_of_val(&s2));
    
    let sb2 = s2.as_bytes();

    println!("The '헬로 월드' bytes size is: {}", sb2.len());
}
