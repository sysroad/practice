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