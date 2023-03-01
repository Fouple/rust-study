/**
 * 编译器采用三条规则来判断引用何时不需要明确的注解。第一条规则适用于输入生命周期，
 * 后两条规则适用于输出生命周期。如果编译器检查完这三条规则后仍然存在没有计算出生
 * 命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。
 *
 * 第一条规则是每一个是引用的参数都有它自己的生命周期参数。换句话说就是，有一个引
 * 用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数
 * 有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
 *
 * fn first_word<'a>(s: &'a str) -> &str {}
 *
 *
 *
 * 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：
 * fn foo<'a>(x: &'a i32) -> &'a i32。
 *
 * fn first_word<'a>(s: &'a str) -> &'a str {}
 *
 *
 *
 * 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self
 * ，说明是个对象的方法(method), 那么所有输出生命周期参数被赋予 self 的生命周期。
 *
 *
 *
 */
// 生命周期 每一个引用都有一个生命周期
// Rust 编译器有一个 借用检查器（borrow checker），它比较作用域来确保所有的借用都是有效的。
use std::f32::consts::E;

// 函数中的泛型生命周期
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("yxz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some year ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("i {:?}", i);
}

// 生命周期注解语法
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 综上，生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。一旦他们形成
// 了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反
// 内存安全的行为。

// 结构体定义中的生命周期注解
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// 这里是一个适用于第三条生命周期省略规则的例子
/**
 * 这里有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予 &self 和
 * announcement 他们各自的生命周期。接着，因为其中一个参数是 &self，返回值类型
 * 被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了。
 */
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命
// 周期被称为 输出生命周期（output lifetimes）
// 被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/**
 * 静态生命周期
 * 这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。
 * 所有的字符串字面值都拥有 'static 生命周期
 */
fn test() {
    let s: &'static str = "I have a static lifetime.";
}

// 结合泛型类型参数、trait bounds 和生命周期
use std::fmt::Display;

/**
 * ann 的类型是泛型 T，它可以被放入任何实现了 where 从句中指定的 Display trait 的类型。
 * 这个额外的参数会使用 {} 打印，这也就是为什么 Display trait bound 是必须的。因为生命
 * 周期也是泛型，所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表中。
 */
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
