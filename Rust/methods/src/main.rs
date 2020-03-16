fn main() {
    let rect1 = Rectangle { width: 50, height: 30 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // `& mut` 로 넘기려면 mut 로 만들어야함
    let mut rect1 = Rectangle { width: 1, height: 5 };
    rect1.set_width(10);
    rect1.set_height(15);
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle { width: 5, height: 5 };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(15);
    println!("square: {:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 메소드 첫번째 파라미터로 `&self` 를 넘긴다
    // `&self` 는 소유권이 넘어가지 않으며 immutable 인 것에 주목
    // mut 을 붙여서 mutable 하게 넘길 수도 있지만 역시 소유권은 넘어가지 않는다
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(& mut self, width: u32) {
        self.width = width;
    }

    fn set_height(& mut self, height: u32) {
        self.height = height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // self 를 파라미터로 갖지 않는 함수도 가능
    // 연관 함수 `associated functions` 라고 부른다
    // static function 으로 보면 되겠다
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
