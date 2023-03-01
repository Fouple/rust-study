pub mod something;

unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;
}

use std::ops::Add;

use crate::something::{Human, Pilot, Wizard, Dog, Animal};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}



/// 不安全 unsafe
/// 解引用裸指针
/// 调用不安全的函数或方法
/// 访问或修改可变静态变量
/// 实现不安全 trait
/// 访问 union 的字段
fn objects() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 += 1;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        dangerous();
        println!("Abs value of -3 to C : {}", abs(-3));
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

