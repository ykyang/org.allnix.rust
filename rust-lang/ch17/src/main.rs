extern crate ch17;

use ch17::AveragedCollection;
use ch17::{Screen, Button, SelectBox};
use ch17::Draw;

fn main() {
    let mut s = String::new();
    s.push_str("Test");

    println!("String: {}", s);

    ch17_2();
}

fn ch17_1() {
    let mut list = AveragedCollection::new();
    list.add(5);
    list.add(3);
    println!("Average: {}", list.average());
    list.remove();
    println!("Average: {}", list.average());
}

fn ch17_2() {
    let mut comps :Vec<Box<Draw>> = Vec::new();
    comps.push(Box::new(SelectBox::new(
        75, 10,
        vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ])
    ));

    comps.push(Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    }));

    let screen = Screen {
        components: comps,
    };

    screen.run();
}