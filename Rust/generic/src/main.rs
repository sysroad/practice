extern crate generic;

use generic::*;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probabley already know, poeple"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
    println!("{}", tweet.summary());

    let weather = WeatherForecast {
        high_temp: 32.1,
        low_temp: 12.6,
        chance_of_precipitation: 44.2
    };

    notify(&weather);
    println!("{}", weather.summary());
}
