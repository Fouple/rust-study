use std::io::{self, Read};
// use std::cmp::Ordering;
// use rand::Rng;

use std::fs::{self, File};
use std::io::ErrorKind;

fn main() {
    // println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop {
    //     println!("Please input you guess: ");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You guessed: {guess}");

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small"),
    //         Ordering::Greater => println!("Too big"),
    //         Ordering::Equal => {
    //             println!("You win");
    //             break;
    //         }
    //     }
    // }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let fw = File::open("world.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let fu = File::open("good.txt").unwrap();

    let fe = File::open("bye.txt").expect("Failed to open bye.txt");
    
}

fn panic_learn() {
    panic!("crash and burn !!!");
}

// 传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // 以上代码等效于以下代码, ? 运算符
    // let mut fi = File::open("world.txt")?;
    // let mut si = String::new();
    // fi.read_to_string(&mut si)?;
    // Ok(si)

    // 以上代码等效于以下代码, ? 运算符
    // let mut sl = String ::new();
    // File::open("good.txt")?.read_to_string(&mut sl)?;
    // Ok(sl)

    // 以上代码等效于以下代码
    fs::read_to_string("hello.txt")

    // ? 可用于 Option<T>, Result<T, E>, 但是不可以混搭使用，? 不会自动将 Result 
    // 转换为 Option
}