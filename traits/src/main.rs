mod basket;
mod stack;
use std::vec;

use container::Container;
use basket::Basket;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
} 

fn main() {
    let mut b1 = Basket::new("hi there".to_string());
    let b2 = Basket::new(42);

    let mut s1 = Stack::new(vec!["hi".to_string()]);
    let s2 = Stack::new(vec![1, 2, 3]);

    add_string(&mut b1, "hello".to_string());
    add_string(&mut s1, "hello".to_string());
}
