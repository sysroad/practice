fn main() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ...
    }

    // match 연산자
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), //미국 주별로 다른 쿼터 동전이 존재
    }

    fn value_in_cents(coin: &Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // 알라스카 발행 쿼터는 희귀하다고 가정하고 30 가치를 주자
                match state {
                    UsState::Alaska => 30,
                    _ => 25,
                }
            }
        }
    }

    fn print_coin_value(coin: Coin) {
        match &coin {
            Coin::Quarter(state) => {
                println!("Quater of {:?}'s value is {}", state, value_in_cents(&coin));
            }
            _ => println!("{:?}'s value is {}", coin, value_in_cents(&coin)),
        }
    }

    print_coin_value(Coin::Penny);
    print_coin_value(Coin::Nickel);
    print_coin_value(Coin::Dime);
    print_coin_value(Coin::Quarter(UsState::Alabama));
    print_coin_value(Coin::Quarter(UsState::Alaska));

    // Option<T> 를 이용한 매칭
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // `if let` 을 사용한 간결한 흐름 제어
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 위의 match 문을 if let 으로 바꾸면
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // if let 은 특정한 하나의 패턴에 매칭 되는 경우 외에는
    // 다른 값은 무시하는 match 문에 대한 syntax sugar 로 볼 수 있다.
    // 물론 else 를 포함 시킬 수도 있다.
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}
