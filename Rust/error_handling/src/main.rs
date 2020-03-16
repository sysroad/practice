use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;

fn main() {
    //panic!("crash and burn");

    let f = File::open("hello.txt");
    //let f = match f {
    //Ok(file) => file,
    // Err(error) => {
    //     panic!("There was a problem opening the file: {:?}", error)
    // },
    //};

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but failed: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    //let f = File::open("hello2.txt").unwrap(); // Result 가 Err 면 panic! 이 호출됨

    //let f = File::open("hello2.txt").expect("Fail Fail Fail"); // expect 는 에러 메시지를 선택 가능
}

// 에러 전파
// 호출 코드로 에러를 반환하는 함수
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello2.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 에러 전파 간략 버전
// `?` : C# 의 nullable operator 와 비슷하게 사용
// 단 Rust 의 `?` 는 반환 타입이 `Result` 인 함수에서만 사용 가능하다
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello2.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 실패가능성이 있는 함수의 경우 `Result` 반환은 기본적으로 좋은 선택지이다
// 프로토타입이나 테스트 목적의 경우 `Result` 보다는 `panic!` 을 던지는 것이 좋다 (문제 부분을 즉각적으로 캐치)
// <<== unwrap , expect 등의 메소드를 호출
// 컴파일러가 컴파일 타임에 확실한 모든 정보를 갖을 수 없는 경우 (사용자 입력으로 받는 ip 주소 등)
// unwrap 사용으로 panic! 을 발생하는 것보다는 Result 를 통해 Err variant 를 처리해주는 것이 나음