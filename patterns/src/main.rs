struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    // 匹配字面值
    let x1 = 1;

    match x1 {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配命名变量
    let x2 = Some(5);
    let y2 = 10;

    match x2 {
        Some(50) => println!("Got 50"),
        Some(y2) => println!("Matched, y2 = {:?}", y2),
        _ => println!("Default case, x2 = {:?}", x2),
    }
    println!("at the end: x2 = {:?}, y2 = {:?}", x1, y2);

    // 多个模式
    let x3 = 1;
    match x3 {
        1 | 2 => println!("one or two: {:?}", x3),
        3 => println!("three: {:?}", x3),
        _ => println!("anything: {:?}", x3),
    }

    // 通过..=匹配值的范围
    let x4 = 5;
    match x4 {
        1..=5 => println!("one throught five: {:?}", x4), // 1..=5 与 1 | 2 | 3 | 4 | 5 相等
        _ => println!("something else: {:?}", x4),
    }

    let x5 = 'c';
    match x5 {
        'a'..='j' => println!("early ASCII letter: {:?}", x5),
        'k'..='z' => println!("late ASCII letter: {:?}", x5),
        _ => println!("something else: {:?}", x5),
    }

    // 解构并分解值
    // 解构结构体
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(p.x, x);
    assert_eq!(p.y, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 解构枚举
    // 结构嵌套的结构体和枚举
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direciton {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
    }

    // 结构结构体和元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // 使用 _ 忽略整个值
    foo(3, 4);

    // 使用嵌套的 _ 忽略部分值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // 通过在名字前以一个下划线开头来忽略未使用的变量
    let s = Some(String::from("Hello"));

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // 用..忽略剩余值
    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (.., last) => {
            println!("Some numbers: {}", last);
        }
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // 匹配守卫提供的额外条件
    // 匹配守卫 是一个指定于 match 分支模式之后的额外 if 条件
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x6 = 6;
    let y6 = true;

    match x6 {
        4..=6 if y6 => println!("yes"),
        _ => println!("no"),
    }

    // @绑定
    enum SMessage {
        Hello { id: i32 },
    }

    let mg = SMessage::Hello { id: 5 };
    match mg {
        SMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        SMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        SMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}
