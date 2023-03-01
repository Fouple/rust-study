/**
 * 如果我们 确实 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，
 * 可以使用一个叫做 clone 的通用函数。
 * 
 * Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上
 *（第十章将会详细讲解 trait）。如果一个类型实现了 Copy trait，那么一个旧的变量
 * 在将其赋值给其他变量后仍然可用。
 */
fn what_is() {
    
    let s1 = gives_ownership(); // s1 进入作用域
    println!("gives_ownership: {}", s1);

    let s2 = String::from("hello world");
    println!("s2: {}", s2);
    // takes_ownership(s2); // s2 的值移动到函数里...
    // println!("s2: {}", s2); // ... 所以到这里不再有效

    let s3 = takes_and_gives_back(s2);
    println!("takes_and_gives_back: {}", s3);

    let x = 5;
    makes_copy(x); // x 应该移动到函数里，但是 i32 是 Copy的,所以在后面可以继续使用x
    println!("x is alive {x}");

    let s4 = String::from("s4 is s4");
    let (s4, len) = tuple_calculate_length(s4);
    println!("The length of '{}' is {}.", s4, len);
} // 在这里，自底向上移出作用域

fn gives_ownership() -> String { // gives_ownership 会将返回值移动给调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用的函数
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 在这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

// 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 在这里，some_integer 移出作用域。

fn tuple_calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}