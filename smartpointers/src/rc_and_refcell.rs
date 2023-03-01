use std::cell::Ref;
use std::{ops::Deref, cell::RefCell};

/// # example
///
/// '''
///     use crate::List::{Cons, Nil};
/// ...
///     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
/// '''
///
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

#[derive(Debug)]
enum OList {
    OCons(Rc<RefCell<i32>>, Rc<OList>),
    ONil,
}

use crate::List::{Cons, Nil};
use crate::OList::{OCons, ONil};
use std::rc::Rc;

/// Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
/// Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；
/// RefCell<T> 允许在运行时执行不可变或可变借用检查。
/// 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是
/// 不可变的情况下修改其内部的值。
fn rc_and_refcell() {
    let x = 5;
    let y = &x;
    let z = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let m = MyBox::new(String::from("Roobin"));
    say_hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);

    let rca = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating rca = {}", Rc::strong_count(&rca));
    let rcb = Cons(3, Rc::clone(&rca));
    println!("count after creating rcb = {}", Rc::strong_count(&rca));
    {
        let rcc = Cons(4, Rc::clone(&rca));
        println!("count after creating rcc = {}", Rc::strong_count(&rca));
    }
    println!("count after rcc goes out of scope = {}", Rc::strong_count(&rca));

    let value = Rc::new(RefCell::new(5));
    let ra = Rc::new(OCons(Rc::clone(&value), Rc::new(ONil)));
    let rb = OCons(Rc::new(RefCell::new(3)), Rc::clone(&ra));
    let rc = OCons(Rc::new(RefCell::new(4)), Rc::clone(&ra));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", ra);
    println!("b after = {:?}", rb);
    println!("c after = {:?}", rc);


}
