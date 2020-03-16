fn main() {
    let mut number = 3;
    
    println!("loop - break 사용");

    loop {

        println!("{}!", number);

        if { number = number - 1; number } <= 0 {
            break
        }
    }

    println!("while 구문 사용");

    number = 3;

    while number > 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("for 문 사용");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    println!("for 문 rev() 사용");

    for number in (1..4).rev() { // [1, 4) => 1, 2, 3 => 3, 2, 1   (4..1) 과는 다름
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
