mod whatis;

/*
所有权、借用和 slice 这些概念让 Rust 程序在编译时确保内存安全。
Rust 语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，
但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须
额外编写和调试相关的控制代码。
 */
fn main() {
    let mut s1 = String::from("hello world");

    let word = old_first_word(&s1); // word = 5

    s1.clear(); // s1 = ""

    // word 仍是5，但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
    
    // slice [0..len] == [..]
    let s2 = String::from("hello world");
    let s2_len = s2.len();

    let hello = &s2[0..5]; // == [..5]
    let world = &s2[6..s2_len]; // == [6..]

    
    let s3 = String::from("hello world");
    
    let word = first_word(&s3[..]);
    // 当拥有某值的不可变引用时，就不能再获取一个可变引用。
    // s3.clear(); error 因为 clear 需要清空 String，它尝试获取一个可变引用。
    println!("the first word is: {}", word);

    // 通用slice, 以下 slice 的类型是&[i32], 可以对其他所有集合使用这类 slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn old_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 字符串 slice 类型：str
// 如果有一个 String，则可以传递整个 String 的 slice 或对 String 的引用。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}