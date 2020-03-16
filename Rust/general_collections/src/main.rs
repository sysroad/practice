fn main() {

    // 벡터
    {
        use std::option::*;

        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);

        // 벡터 생성 편의성을 위한 매크로 `vec!`
        let v = vec![1, 2, 3];

        for i in &v {
            print!("{} ", i);
        }
        println!();
        // for 문에서 `&v` 대신 `v` 를 쓰면 소유권이 넘어가고 for 문 이후 v 는 무효화 된다
        println!("{:?}", v);

        // 벡터의 요소 읽기
        let third: &i32 = &v[2];
        let third: Option<&i32> = v.get(2);

        // let does_not_exist = &v[100]; // panic 발생 
        let does_not_exist = v.get(100);
        if does_not_exist == Option::None {
            println!("not exist");
        }
        /*
            `&v[..]` 는 범위를 넘어서는 접근을 허용하지 않으며, 잘못된 접근이 있을 경우 프로그램을 죽게끔 하는게 나을 경우 사용
            `v.get(..)` 는 범위를 넘어서는 요청을 적절히 걸러내어 따로 처리할 수 있는 경우 사용`=
        */

        // 벡터 내의 값들에 대한 반복
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50; // 값에 접근하기 위해 `*` 역참조 연산자를 사용한다
            print!("{} ", i);
        }
        println!();
        for i in &v {
            print!("{} ", i);
        }
        println!();

        // 열거형 활용하기
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    // 문자열 String , UTF-8 Encoding
    {
        // str : 스트링 슬라이스
        // &str : 스트링 스라이스에 대한 참조자
        // 스트링 슬라이스 : 어딘가에 저장된 UTF-8 로 인코딩된 스트링 데이터의 참조자
        // 스트링 리터럴 -> 프로그램 바이너리 출력물의 어딘가에 내장되어 있음 == 스트링 슬라이스

        // String : 러스트 표준 라이브러리를 통해 제공된다
        // 가변적이며 UTF-8 로 인코딩된 스트링 타입이다
        // 보통 `문자열` 이라하면 `String` 과 `&str` 둘다를 포괄하여 지칭함

        // 새로운 스트링 생성 `new` 사용
        let mut s = String::new();

        // UTF-8 인코딩
        let hello = String::from("Hello");
        println!("{}", hello);
        let hello = String::from("Здравствуйте"); // ok
        println!("{}", hello);
        let hello = String::from("안녕하세요"); // ok
        println!("{}", hello);

        // 문자열 추가
        let mut s = String::from("foo");
        let s2 = "bar";
        s.push_str(s2); // push_str 은 소유권을 가져가지 않는다
        println!("s2 is {}", s2);

        // `+` 연산자 , `format!` 매크로 사용
        let s1 = String::from("Hello, ");
        let s2 = String::from("World!");

        //let s3 = s1 + s2; // expected `&str`
        // + 연산자는 add 를 사용한다
        // fn add(self, s: &str) -> String { ... 같은 형태 (표준 라이브러리는 제너릭으로 정의되어있다)
        // s1 은 여기서 이동된다, 
        // &String 인자는 &str 로 강제될 수 있다 <- 역참조 강제 `deref coercion`
        // add 가 소유권을 가져가지 않으므로 s2 는 add 이후에도 유효하다
        let s3 = s1 + &s2; 

        //println!("s1 is {}", s1); // borrow of moved value `s1`
        println!("s3 is '{}'", s3);

        //복잡한 조합에는 `format!` 사용
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        // format! 은 파라미터의 소유권을 가져가지 않는다
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s is '{}'", s);

        //내부 인덱싱
        let s1 = String::from("hello");
        //let h = s1[0]; // error : `std::string::String` cannot be indexed by integer
        
        let mut chs = s1.chars();
        let x = chs.nth(0).unwrap(); 
        // nth(0) 을 다시 호출하면 `h` 가 아닌 `e` 가 나온다
        // chs 내부적으로 이터레이팅 시키는듯
        println!("{}", x);

        //스트링 내부 순회
        for c in "नमस्ते".chars() {
            print!("{} ", c);
        }
        println!();
        let mut chs = "नमस्ते".chars();
        loop {
            let x = chs.next();
            if x == Option::None {
                break;
            }
            print!("{} ", x.unwrap());
        }
        println!();

        let c : char = 'c';
        //let c : char = 'ते'; // error

        /*
            rust 의 문자열 데이터는 C++ 처럼 byte 의 배열이다.
            C# 처럼 char(UTF-16) 의 컬렉션을 들고 있는게 아니다.
         */

        // byte 로 반환
        for b in "नमस्ते".bytes() {
            print!("{} ", b);
        }
        println!();

        // chars() 는 유니코드 캐릭터 단위로 쪼개주지는 않는다
        // `unicode-segmentation` crate 를 참고해보자
        // https://crates.io/crates/unicode-segmentation
    }

    // 해쉬맵 `HashMap<K, V>`
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let mut codes : HashMap<i32, i32> = HashMap::new();

        codes.insert(0, 10);
        codes.insert(1, 40);

        // 튜플의 벡터에 대해 collect 메소드를 사용하여 해시맵 생성
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        // collect 리턴 타입이 제너릭 타입임.
        // 컴파일러에게 명시적으로 알려줄 필요가 있음. HashMap<_, _>
        // `_` <- 벡터에 담긴 데이터 타입에 기반하여 추론하도록 함
        // zip : 두개의 반복자가 같이 순회하게끔 해줌
        // collect : 반복자를 컬렉션으로 변환해줌
        // iter1().zip(iter2()).collect() --> (iter1, iter2) 튜플의 컬렉션 반환
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

        // Copy Trait 을 구현한 타입은 값들이 해시맵으로 복사된다.
        // String 처럼 소유된 값들은 값이 이동되어 소유권이 넘어간다.
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        //이제 field_name, field_value 는 유효하지 않다
        //println!("{} : {}", field_name, field_value); // error

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        //복제본을 만들어 소유권을 맵으로 넘김
        map.insert(field_name.clone(), field_value.clone()); 
        println!("{} : {}", field_name, field_value); // ok
        
        for (key, value) in &scores {
            println!("{} : {}", key, value);
        }

        //해시맵 내의 값 접근하기
        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        if score != Option::None {
            println!("Blue score is : {}", score.unwrap());
        }

        // or_insert : 키가 존재하면 해당 값을 반환하고 없으면 새 값을 삽입
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0); // or_insert 는 가변 참조자 `&mut V` 를 반환한다
            *count += 1; // 값 할당을 위해 역참조 사용
        }

        println!("{:?}", map);
    }
}
