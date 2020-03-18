fn main() {
    let x = "abcd";
    let y = "123";
    println!("{}", longest(x, y));

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("{}", result);
    }

    // 정적 라이프타임
    // 모든 스트링 리터럴은 정적 라이프 타임을 갖는다
    // 프로그램 종료때까지 유효
    // 'static 지정은 댕글링 참조에 대한 해결책이 아니다.. 무분별한 사용 자제
    let s: &'static str = "I have a static lifetime";
}

/*
라이프타임 명시는 어퍼스트로피(') 로 시작하고
파라미터 이름은 보통 모두 소문자다.
라이프 타임 명시는 참조자의 & 뒤에 위치하고
공백으로 구분한다
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
