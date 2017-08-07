use std::vec::Vec;
//use std::Option::Some;

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection { list: Vec::new(), average: 0.}
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn remove(&mut self) -> Option<i32> {
        let ans = self.list.pop();
        match ans {
            Some(value) =>{
                // > It may not be desirable to update
                // > automatically, in the case where user
                // > want to pop several items.
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    fn update_average(&mut self) {
        let sum :i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button::draw()");
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> SelectBox {
        SelectBox {
            width: width,
            height: height,
            options: options,
        }
    }
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox::draw()");
    }
}

// --> Chapter 17.3 <--
pub struct Post {
    state: Option<Box<State>>,
    content: String,
}
impl Post {
    pub fn new() -> Post {
        return Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
         self.state.as_ref().unwrap().content(&self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
 }
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<State> {
        // > Draft -> Draft
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published{})
    }
}

struct Published{}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }


    #[test]
    fn test_post() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!(post.content().len(), 0);
    }
}
