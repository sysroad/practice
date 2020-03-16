#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 열거형의 타입 지정이 가능하다
#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}

// 각기 다른 타입과 다른 갯수의 연관 데이터를 가질 수 있다
#[derive(Debug)]
enum IpAddress2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    route(v4);
    route(v6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home address is `{:?}`", home);
    println!("loopback address is `{:?}`", loopback);

    // 더 간결한 표현으로
    // 러스트의 열거형은 타입 지정이 가능하고
    // 각기 다른 타입과 다른 갯수의 연관데이터를 갖는 것이 가능하다
    let home = IpAddress::V4(String::from("127.0.0.1"));
    println!("home address is `{:?}`", home);
    let home = IpAddress2::V4(127, 0, 0, 1);
    println!("home address is `{:?}`", home);

    // 모든 variants 는 Message 타입으로 그룹화된다
    enum Message {
        Quit,                       // 연관 데이터가 없다   == struct QuitMessage; // 유닛 구조체
        Move { x: i32, y: i32 },    // 익명 구조체          == struct MoveMessage { x: i32, y: i32 } // 구조체
        Write(String),              // String               == struct WriteMessage(String); // 튜플 구조체
        ChangeColor(i32, i32, i32), // i32 세개             == struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체
    }

    // 열거형도 메소드 정의가 가능하다
    impl Message {
        fn call(&self) {
            match self {
                Message::Write(s) => println!("{}", s),
                Message::Move { x, y } => println!("move to ({}, {})", x, y),
                Message::ChangeColor(r, g, b) => println!("r: {}, g: {}, b: {}", r, g, b),
                Message::Quit => println!("quit"),
            }
        }
    }

    let m = Message::Write(String::from("hello world"));
    m.call();
    let m = Message::Move { x: 100, y: 200 };
    m.call();
    let m = Message::ChangeColor(120, 243, 159);
    m.call();
    let m = Message::Quit;
    m.call();

    /* 표준 라이브러리의 `Option` 열거형
    러스트에는 Null 타입이 없다
    null 의 존재와 null 참조로 발생하는 문제들
        --> 수많은 오류, 취약점들 및 시스템 종료 유발
    러스트는 null 이 표현하고자 했던 개념
    `값의 부재` 개념을 열거형 Option 으로 표현한다.

        enum Option<T> {
            Some(T),
            None,
        }
    
    Option 은 명시적으로 가져오지 않고도 사용가능하다.
        --> `Option::` 생략 가능
    */
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; // None 사용시는 타입을 명시적으로 알려줄 필요가 있다

    // Option<T> 와 T 는 다른 타입이며 컴파일러는 Option<T> 의 값을 유효값으로 사용하는 것을 막는다
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y; // error: no implementation for `i8 + std::option::Option<i8>`
    // i8 과 같은 타입은 항상 유효한 값을 갖고 있다는 것이 보장된다.
    // 하지만 Option<i8> 은 항상 유효한 값이 있다는 보장이 없다.
    //  --> 연산 수행전 Option<T> 에서 T 로 변환이 필요하다.

    // << direction >>
    // null(값의 부재) 일 수 있는 값을 사용할 때는 명시적으로 값의 타입을 Option<T> 로 만들 것.
    // 값을 사용할 때는 명시적으로 null 인 경우를 처리할 것.
    // 이를 통해 값 타입이 Option<T> 가 아닌 모든 곳에서 값이 null 이 아니라고 가정할 수 있다.
    let sum = x + y.unwrap_or_default();
    println!("x + y: {}", sum);
}

fn route(ip_type: IpAddrKind) {
    println!("select ip type: {:?}", ip_type);
}
