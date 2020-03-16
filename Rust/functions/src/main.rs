fn main() {

    let x = 5;
    let y = { // block {} 은 표현식이다
        let x = 3;
        x + 1 // 표현식은 종결을 나타내는 세미콜론을 사용하지 않는다
    };

    another_function(x, y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(x);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 { // 반환 타입을 명시한다
    5 // 암묵적으로 마지막 표현식을 반환한다
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 세미콜론을 사용하지 않는다
}