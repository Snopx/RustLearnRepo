pub mod learn_trait {
    use std::string;

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("a o ,this default impl")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub fn notify<T: Summary>(item: &T) {
        println!("notify:{}", item.summarize());
    }

    pub fn get_impl_summary() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    // note: 使用 impl trait 作为返回值时候 只能返回确定的一种类型，返回不同类型会报错 👆
}
