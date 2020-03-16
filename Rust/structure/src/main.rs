struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("someone123"),
        email: String::from("someone123@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 name: {}", user1.username);
    println!("user1 email: {}", user1.email);
    println!("user1 active: {}", user1.active);
    println!("user1 sign_in_count: {}", user1.sign_in_count);

    // 수정 가능한 변수 user2 를 만들고 active 를 false 로 변경
    let mut user2 = build_user("someone234", "someone234@example.com");
    user2.active = false;
    user2.sign_in_count = 10;

    // user2 로 부터 나머지 필드를 받아오는 user3 를 만든다
    let user3 = from_user("someone345", "someone345@example.com", &user2);

    user2.active = true;
    user2.sign_in_count = 1;

    println!("user3 name: {}", user3.username);
    println!("user3 email: {}", user3.email);
    println!("user3 active: {}", user3.active); // false 가 출력됨
    println!("user3 sign_in_count: {}", user3.sign_in_count); // 10

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);

    // unit-like structs
    // 필드가 없는 구조체, 유닛 타입인 `()` 과 비슷하게 동작
    // 특정한 타입의 trait 을 구현해야 하지만 타입 자체에 데이터를 저장하지 않는 경우 유용

    // 구조체 데이터의 소유권
    // struct RefUser {
    //     username: &str, // error: missing lifetime specifier
    //     email: &str, // error: missing lifetime specifier
    //     sign_in_count: u64,
    //     active: bool,
    // }

    // let user1 = RefUser {
    //     username: "someone",
    //     email: "someone@example.com",
    //     active: true,
    //     sign_in_count: 1,
    // };

    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));

    // 파생 트레잇 `derived trait` 으로 기능 추가하기
    //println!("rect1 is {}", rect1);   // error: `Rectangle` doesn't implement `std::fmt::Display`
    println!("rect1 is {:?}", rect1);   // `:?` 은 Debug 출력 포맷을 사용하라는 의미
                                        // Debug 출력을 위해 구조체 선언 앞에 `#[derive(Debug)] ` 를 추가한다
                                        // "rect1 is Rectangle { width: 50, height: 30 }"
    println!("rect1 is {:#?}", rect1);  // `{:#?}` 개행/들여쓰기 된 상태로 출력

    /*
    automatic referencing and dereferencing

    러스트는 포인터에 대한 역참조 연산자인 `->` 가 없다.
    some_object.do_something() 호출시 러스트는 자동으로 `&` 나 `&mut` 또는 `*` 를 붙여서
    해당 메소드의 시그니처와 일치하도록 만들어준다.
    즉, 러스트는 참조/역참조 동작을 적절히 알아서 판단한다.
    이것은 메소드가 명확한 수신자 `self` 를 갖기 때문에 가능하다.
    */
}

#[derive(Debug)] // 디버깅 정보 출력이 가능하도록 한다
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(username: &str, email: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1,
    }
}

fn from_user(username: &str, email: &str, user: &User) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        ..*user
    }
}