#![allow(unused)]

/**
 * 现在我们涉及到了如何使用枚举来创建有一系列可列举值的自定义类型。我们也展示了标准库的
 * Option<T> 类型是如何帮助你利用类型系统来避免出错的。当枚举值包含数据时，你可以根据
 * 需要处理多少情况来选择使用 match 或 if let 来获取并使用这些值。
 * 
 * 你的 Rust 程序现在能够使用结构体和枚举在自己的作用域内表现其内容了。在你的 API 中
 * 使用自定义类型保证了类型安全：编译器会确保你的函数只会得到它期望的类型的值。
 */

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do call
    }
}

// match 控制流结构
enum Yuan {
    One,
    Five,
    Ten,
    Twenty,
    Fifty,
    OneHundred,
}

fn value_in_yuan(yuan: Yuan) -> u8 {
    match yuan {
        Yuan::One => 1,
        Yuan::Five => 5,
        Yuan::Ten => 10,
        Yuan::Twenty => 20,
        Yuan::Fifty => 50,
        Yuan::OneHundred => {
            println!("是 100 毛爷爷捏!");
            100
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home_ip_addr = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home_ip_addr, loopback);

    let message = Message::Write(String::from("hello"));
    message.call();

    // 空值是一个因为某种原因目前无效或缺失的值。
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);

    println!("{}", value_in_yuan(Yuan::OneHundred));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?}, {:?}", five, six, none);

    // 通配模式
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // _ 占位符, 可以匹配任意值而不绑定到该值
    let dice_change_roll = 9;
    match dice_change_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    let dice_nothing_roll = 10;
    match dice_nothing_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // if let 相当于 match 的语法糖
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}

fn reroll() {}