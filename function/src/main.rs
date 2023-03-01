fn main() {
    // Hello, world!
    println!("Hello, world!");

    another_function('F', 12);

    // 表达式会计算出一个值，并且你将编写的大部分 Rust 代码是由表达式组成的。
    let y = {
        let x = 3;
        x + 1 // 表达式的结尾没有分号
    };
    // 如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
    println!("The value of y is: {y}");
    
    let number = 19;
    if number == five() {
        let x :i32 = five();
        println!("x is {x}");
    } else if number > five() {
        println!("number 大于 5 了哇");
    } else if number < five() {
        println!("number 小于 5 了哇");
    } else {
        println!("不知道哪里出错😅了哇");
    }

    if number % five() == 0 {
        println!("{number} 是 {} 的倍数。", five());
    }

    let x: i32 = plus_one(10);
    println!("x is {x}");

    // if 是表达式, 所以还可以这样子
    let condition = true;
    let number = if condition { 5 } else {6};
    println!("number is {number}");

    test_loop(number);
    double_loop(9);

    let a = [10, 20, 30, 40, 50];
    test_while(a);
    test_for(a);


}


/**
 * 函数测试1
 */
fn another_function(prefix: char, x: i32) {
    println!("This is another function, x is: {prefix}{x}");
}

/**
 * 函数测试2
 */
fn five() -> i32 {
    5
}

/**
 * 函数测试3
 */
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn test_loop(n: i32) {
    println!("Test loop start");
    let mut i = 0;
    let result = loop {
        if i == n { break i; }
        i += 1;
        println!("time: {i}");
    };
    println!("loop end, time: {result}");
}

fn double_loop(n: i32) {
    println!("Double loop start");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == n {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Double loop end, count = {count}");
}

fn test_while(a: [i32; 5]) {
    let mut index = 0;
    println!("while start");
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("while end");

}

fn test_for(a: [i32; 5]) {
    println!("for start 1");
    for element in a {
        println!("the value is: {element}");
    }
    println!("for end 1");

    println!("for start 2");
    for number in 1..4 {
        println!("the value is: {number}");
    }
    println!("for end 2");
}