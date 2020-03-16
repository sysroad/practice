fn main() {

    // 소유권의 이동
    {
        let s1 = String::from("hello");
        let s2 = s1;
    
        //println!("{}, world", s1); // error : borrow of moved value:`s1`
        println!("{}, world", s2);
    
        let s3 = s2.clone();
    
        println!("s2 = {}, s3 = {}", s2, s3);
    
        let mut x = 5;
        let y = x;
        x = 2;
    
        println!("x = {}, y = {}", x, y);
    }

    // 소유권과 함수
    {
        let s = String::from("hello");

        takes_ownership(s);

        //println!("s is: {}", s); // error: value borrowed after move
    
        let x = 5;
    
        makes_copy(x);
    } // s 는 이미 이동되었으므로 별다른 일이 발생하지 않는다

    // 반환 값과 스코프
    {
        // 값의 반환도 소유권을 이동시킴

        let s1 = gives_ownership(); // 반환값을 s1 에게 이동

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2); // s2 가 함수 안으로 이동되었고
                                           // 반환값을 s3 로 이동시킴
    } // s1 과 s3 는 drop 이 호출됨
      // s2 는 이동되었으므로 아무 일도 없음
      // 소유권을 이동 시켜주지 않으면 스코프를 벗어나면서 drop 에 의해 제거된다
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // scope 종료 시점에서 some_string 의 drop 이 호출됨

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}