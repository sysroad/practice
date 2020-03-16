fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // 표현식의 반환 타입은 동일해야 한다

    println!("The value of number is: {}", number);
}
