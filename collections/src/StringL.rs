/// Rust 的核心语言中只有一种字符串类型：字符串slice str，它通常以被借用的
/// 形式出现，&str。
///
/// 称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、
/// 有所有权的、UTF-8 编码的字符串类型。当 Rustacean 们谈到 Rust 的 “字符串”时，它们
/// 通常指的是 String 或字符串slice &str 类型，而不特指其中某一个。
///
/// String 和字符串 slices 都是 UTF-8 编码的。

fn StringL() {
    let data = "initial contents";

    let _s = data.to_string();
    let s1 = "initial contents".to_string();
    let s2 = String::from("initial contents");

    let mut hellos: Vec<String> = Vec::new();
    hellos.push(String::from("السلام عليكم"));
    hellos.push(String::from("Dobrý den"));
    hellos.push(String::from("Hello"));
    hellos.push(String::from("שָׁלוֹם"));
    hellos.push(String::from("नमस्ते"));
    hellos.push(String::from("こんにちは"));
    hellos.push(String::from("안녕하세요"));
    hellos.push(String::from("你好"));
    hellos.push(String::from("Olá"));
    hellos.push(String::from("Здравствуйте"));
    hellos.push(String::from("Hola"));

    for s in &mut hellos {
        println!("hello: {}", s);
    }    

    let mut s_test = String::from("foo");
    s_test.push_str(" bar");
    println!("s_test is {}", s_test);
    s_test.push('h');
    println!("s_test is {}", s_test);

    let s3 = s1 + ", " + &s2; // 注意 s1 被移动了，不能继续使用
    // + 运算符看起来像这样 fn add(self, s: &str) -> String {
    // &String 可以被 强转（coerced）成 &str
    
    println!("s3: {}", s3);

    let s4 = format!("{}{}{}", s3, ". ", hellos[2]);
    println!("s4: {}", s4);

    println!("{}", hellos[2]);

    // Rust 的字符串不支持索引。
    // 一个字符串字节值的索引并不总是对应一个有效的 Unicode 标量值
    // 从 Rust 的角度来讲，事实上有三种相关方式可以理解字符串：字节、标量值和字形簇

    // 比如这个用梵文书写的印度语单词 “नमस्ते”，最终它储存在 vector 中的 u8 值看起来像这样：
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    
    // 如果从 Unicode 标量值的角度理解它们，也就像 Rust 的 char 类型那样，这些字节看起来像这样：
    // ['न', 'म', 'स', '्', 'त', 'े']

    // 最后，如果以字形簇的角度理解，就会得到人们所说的构成这个单词的四个字母：
    // ["न", "म", "स्", "ते"]

    // 最后一个 Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间
    // (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来
    // 确定有多少有效的字符。

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }


}
