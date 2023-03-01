// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。

fn reference() {
    let s1 = String::from("this is s1");

    let len = calculate_length(&s1);

    println!("{}, the length is {}", s1, len);

    let mut s2 = String::from("this is s2");
    change(&mut s2);

    // 在同一时间只能有一个对某一特定数据的【可变】引用，以避免数据竞争
    // let r1 = &mut s2;
    // let r2 = & mut s2;

    // 不能在拥有不可变引用的同时拥有可变引用。
    // let r1 = &s2;
    // let r2 = &s2;
    // let r3 = &mut s2; // error
    
    // 编译器在作用域结束之前判断不再使用的引用的能力被称为 非词法作用域生命周期。
    let r1 = &s2;
    let r2 = &s2;
    println!("{} and {}", r1, r2);
    // 此位置后 r1 和 r2 不再使用
    let r3 = &mut s2;
    println!("{}", r3);

    // let reference_to_nothing = dangle();
}

// 引用默认不可变
fn calculate_length(s: &String) -> usize { // s is a reference to String
    s.len()
} // 在这里，s离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发送

fn change(some_string: &mut String) {
    some_string.push_str(", is change");
}

// // 悬垂指针是其指向的内存可能已经被分配给其它持有者。
// fn dangle() -> &String { // dangle 返回一个字符串的引用
//     let s = String::from("hello"); // s 被创建

//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。危险！