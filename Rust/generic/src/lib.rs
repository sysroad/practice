use std::fmt::*;

pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct WeatherForecast {
    pub high_temp: f64,
    pub low_temp: f64,
    pub chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}°C, and the low will be {}°C, The chance of precipitation is {}%.", self.high_temp, self.low_temp, self.chance_of_precipitation)
    }
}

pub fn notify<T: Summarizable>(item: &T) {
    println!("Breaking news! {}", item.summary());
}

// where 절을 사용하여 표현할 수 있다
pub fn some_function<T, U>(t: T, u: U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug
{
    0
}

// largest<T: PartialOrd>
// >> cannot move out of type `[T]`, a non-copy array 에러 발생
// >> T 가 Copy 트레잇을 구현하지 않은 타입일 가능성이 있고
//    list[0] 의 값을 largest 변수로 소유권을 옮기지 못함을 의미
// T 가 Copy 트레잇 대신 Clone 트레잇 바운드를 갖도록 명시하여
// largest 함수가 소유권을 갖길 원하는 경우 슬라이스의 각 값이 복제되도록
// 할 수 있음. 이 경우 clone 사용으로 더 많은 힙 할당이 일어날 수 있고
// 힙할당은 많은 양의 데이터 사용시 느려질 수 있음
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// `T` 대신 `&T` 를 반환하면 Copy 나 Clone 트레잇 바운드가 필요없다
pub fn largets2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}