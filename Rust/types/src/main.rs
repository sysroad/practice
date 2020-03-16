use std::mem::*;

fn main() {
    println!("'char' size is: {}", size_of::<char>());

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is: {}", y);

    let v = vec![1,2,3,4,5];
    
    for val in v.iter() {
        println!("{}", val);
    }
}
