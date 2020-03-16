extern crate communicator;

use communicator::client;
use communicator::server;

/*
    가시성
    - 공개로 표시되면 부모 모듈의 어디서건 접근이 가능하다.
    - 비공개인 경우 동일 파일 내의 부모 모듈 및 같은 부모 모듈에서
      파생된 자식 모듈들에서만 접근이 가능하다.
*/

fn main() {
    println!("hello, world");

    //communicator::client::connect(("127.0.0.1", 8080));
    client::connect(("127.0.0.1", 8080));
    //communicator::server::bind(("127.0.0.1", 8080));
    server::bind(("127.0.0.1", 8080));
    // 인텔리센스가 서브 모듈의 메소드를 찾지 못하지만 호출에는 문제 없음.
    communicator::server::tcp::connect();
    communicator::server::udp::connect();

    enum TrafficLight {
        Red, Yellow, Green,
    }

    // 이름 가져오기
    {
        use TrafficLight::{ Red, Yellow };

        let red = Red;
        let yellow = Yellow;
        let green = TrafficLight::Green;
    }
    
    // `*` 로 모두 가져오기
    {
        use TrafficLight::*;

        let red = Red;
        let yellow = Yellow;
        let green = Green;
    }
}