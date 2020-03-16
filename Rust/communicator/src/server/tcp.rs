pub fn connect() {
    println!("tcp::connect() called");

    // `super` 를 통해 부모 모듈에 접근한다.
    // C# 의 `base` 와 비슷
    super::bind(("127.0.0.1", 8080));

    // `super` 에서 모두 가져온다.
    {
        use super::*;

        bind(("127.0.0.1", 8080));
    }
}

// 서브 모듈들 //
mod bar;