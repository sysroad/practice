pub fn bind(address: (&str, u16)) {
    println!("bind to {}:{}", address.0, address.1);
}

/*
    서브 모듈을 갖는 경우
    서브 모듈은 부모 모듈 이름의 디렉토리 아래에 위치해야 한다
*/

// 서브 모듈들 //
// pub modifier 를 붙여 외부로 노출시킬 수 있다
pub mod tcp; // `./server/tcp.rs`
pub mod udp; // `./server/usp.rs`