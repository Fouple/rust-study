fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

/// 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。能匹配任何传递的可能值的模式被称
/// 为是 不可反驳的（irrefutable）。一个例子就是 let x = 5; 语句中的 x，因为 x 可以匹配任何值所以不可能
/// 会失败。对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）。一个这样的例子便是 
/// if let Some(x) = a_value 表达式中的 Some(x)；如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 
/// 模式不能匹配。
/// 函数参数、 let 语句和 for 循环只能接受不可反驳的模式，因为通过不匹配的值程序无法进行有意义的工作。
/// if let 和 while let 表达式被限制为只能接受可反驳的模式，因为根据定义他们意在处理可能的失败：条件表达式
/// 的功能就是根据成功或失败执行不同的操作。
fn patterns() {
    // 下列列表清单展示了也可以组合并匹配 if let、else if 和 else if let 表达式。这相比 match
    // 表达式一次只能将一个值与模式比较提供了更多灵活性；一系列 if let、else if、else if let 分
    // 支并不要求其条件相互关联。
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // 一个与 if let 结构类似的是 while let 条件循环，它允许只要模式匹配就一直进行 while
    // 循环。下列代码清单展示了一个使用 while let 的例子，它使用 vector 作为栈并以先进后出
    // 的方式打印出 vector 中的值
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 下列代码清单展示了如何使用 for 循环来解构，或拆开一个元组作为 for 循环的一部分
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (_i, _o) = (1, 2);
    // 可以在函数参数中匹配元组
    let point = (3, 5);
    print_coordinates(&point);

}
