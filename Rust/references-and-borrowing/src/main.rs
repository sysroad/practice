fn main() {
    // 참조자
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1); // 참조는 소유권을 넘기지 않는다
    
        println!("The length of '{}' is: {}", s1, len);
    
        let s = String::from("hello");
    
        change(&s);
    }
    
    // 가변 참조자
    {
        let mut s = String::from("hello");

        change2(&mut s);

        println!("{}", s);

        let r1 = &mut s;
        //let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
        //스코프 내에서 s 에 대한 가변 참조는 한개만 만들 수 있다 --> 컴파일 타임에 data race 를 방지해 준다
        //rust 는 data race 가 발생할 수 있는 코드를 컴파일 시점에서 차단한다
        //println!("r1: {} , r2: {}", r1, r2);
        println!("r1: {}", r1);
    }

    // mutable, immutable 참조 혼용
    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        //let r3 = &mut s; // error: cannot borrow `s` as mutable because it is also borrowed as immutable
        //스코프 내에서 `s` 에 대한 immutable 참조가 있다면 mutable 참조를 만들 수 없다
        //여러개의 immutable 참조를 만드는 것은 가능

        //println!("{}, {}, {}", r1, r2, r3);
        println!("{}, {}", r1, r2);
    }

    //허상 참조 Dangling References
    {
        // error: this function's return type contains a borrowed value
        //        , but there is no value
        //let reference_to_nothing = dangle();

        let s = no_dangle();

        println!("s: {}", s);
    }
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 소유권이 밖으로 이동되므로 문제없음
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // `s` 는 여기서 할당 해제 된다
     // 할당 해제 되버리는 `s` 의 참조를 반환하려고 했으므로 컴파일 에러가 난다

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn change(some_string: &String) {
    // some_string.push_str(", world"); // error: 변수가 기본적으로 불변인 것처럼 참조도 불변이다
    println!("{}, world", some_string);
}

fn calculate_length(s: &String) -> usize { // 참조자를 파라미터로 만드는 것을 Borrowing 이라고 한다
    s.len()
} // s 는 참조자이고 소유권이 없으므로 아무일도 발생하지 않는다
