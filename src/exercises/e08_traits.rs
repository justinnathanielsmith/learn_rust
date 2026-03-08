//! Topic 8: Traits and Generics
//!
//! Traits are a powerful way to define shared behavior in Rust. 
//! Generics allow us to write code that works with multiple types.

/// A trait that defines a way to summarize content.
pub trait Summary {
    /// Returns a string summary of the item.
    fn summarize(&self) -> String;
}

/// A struct representing a news article.
pub struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    /// Should return: "{headline}, by {author} ({location})"
    fn summarize(&self) -> String {
        todo!("Implement summarize for Article")
    }
}

/// A struct representing a tweet.
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /// Should return: "{username}: {content}"
    fn summarize(&self) -> String {
        todo!("Implement summarize for Tweet")
    }
}

/// A generic function that takes any item that implements Summary and returns its summary string.
pub fn get_summary<T: Summary>(item: &T) -> String {
    todo!("Implement get_summary")
}

/// A struct representing a point in 2D space.
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Implement the `std::fmt::Display` trait for Point.
/// It should format the point as "(x, y)".
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Implement Display for Point")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_summary() {
        let article = Article {
            headline: String::from("Rust 1.76 Released"),
            location: String::from("Cloud"),
            author: String::from("The Rust Team"),
            content: String::from("Lots of new features..."),
        };
        assert_eq!(
            article.summarize(),
            "Rust 1.76 Released, by The Rust Team (Cloud)"
        );
    }

    #[test]
    fn test_tweet_summary() {
        let tweet = Tweet {
            username: String::from("rustlang"),
            content: String::from("Traits are cool!"),
            reply: false,
            retweet: false,
        };
        assert_eq!(tweet.summarize(), "rustlang: Traits are cool!");
    }

    #[test]
    fn test_get_summary() {
        let tweet = Tweet {
            username: String::from("rustlang"),
            content: String::from("Generics are also cool!"),
            reply: false,
            retweet: false,
        };
        assert_eq!(get_summary(&tweet), "rustlang: Generics are also cool!");
    }

    #[test]
    fn test_point_display() {
        let p = Point { x: 10, y: 20 };
        assert_eq!(format!("{}", p), "(10, 20)");
    }
}
