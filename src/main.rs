// CLI news headlines display

use std::io;
use std::error::Error;
use dotenv::dotenv;
use colour::{yellow, blue};
use newsapi::{Articles, get_articles};



fn render_articles(articles: &Articles, n: usize) {
    for i in 1..n + 1 {
        yellow!("> {}\n", &articles.articles[i].title);
        blue!("> {}\n\n", &articles.articles[i].url);
    }
}

fn readline_string_clean() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    if let Some('\n')= s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')= s.chars().next_back() {
        s.pop();
    }
    return s;
}   

fn main() -> Result<(), Box<dyn Error>>{
    dotenv()?;
    let api_key: String = std::env::var("API_KEY")?;

    print!("Enter 2-letter ISO 3166-1 country code:  ");
    let country_code: String = readline_string_clean();
    let url: String = format!("https://newsapi.org/v2/top-headlines?country={}&apiKey=", country_code);
    let articles: Articles = get_articles(&url)?;
    render_articles(&articles, num_results);
    Ok(())
}
