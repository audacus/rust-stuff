use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    {
        let mut x = 5;
        let y = &mut x;
    }

    let value_5 = Rc::new(RefCell::new(5));
    let value_10 = Rc::new(RefCell::new(10));

    let a = Rc::new(Cons(Rc::clone(&value_5), Rc::new(Cons(Rc::clone(&value_10), Rc::new(Nil)))));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value_5.borrow_mut() += 10;
    *value_10.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b bfter = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
