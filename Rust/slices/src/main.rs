fn main() {
    // 첫단어를 인덱스를 통해 알아내기
    let s = &String::from("hello world");
    let i = indexof_first_word_end(&s);
    println!("{}", &s[..i]);

    // String Slice
    let s = "이제 우리에게 스트링의 첫번째 단어의 끝부분의 인덱스를 찾아낼 방법이 생겼습니다. ";
    // s 의 타입은 &str (string slice) 이다.
    // &str 은 immutable ref 이고 string literal 역시 immutable 이다.
    
    // 한 단어씩 출력해본다.
    let mut tup = get_word(&s);
    loop {
        if tup.1 == "" {
            println!("{}", tup.0);
            break;
        }

        println!("{}", tup.0);
        tup = get_word(tup.1);
    }

    let my_string = String::from("hello world");
    let word = get_word(&my_string[..]); // String 의 슬라이스로 넘김
    println!("word: {}", word.0);

    let my_string_literal = "hello world";
    let word = get_word(&my_string_literal[..]); // 스트링 리터럴의 슬라이스로 동작
    let word = get_word(my_string_literal); // 스트링 리터럴은 자체로 스트링 슬라이스이기도함

    // 배열의 일부를 참조
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    let len = slice.len();

}

fn get_word(s: &str) -> (&str, &str) { // &str : string slice type
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[..i], &s[i+1..])
        }
    }

    (&s[..],&""[..])
}

// 단어 끝부분 인덱스를 반환한다
fn indexof_first_word_end(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
