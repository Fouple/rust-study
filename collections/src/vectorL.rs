fn vectorL() {
    /**
     * Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要
     * 多少内存。第二个好处是可以准确的知道这个 vector 中允许什么类型。如果 Rust 允许
     *  vector 存放任意类型，那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造
     * 成错误。使用枚举外加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况
     */
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let mut lv = vec![1, 2, 3, 4, 5];

    let third: &i32 = &lv[2];
    println!("The third: {}", third);

    match lv.get(2) {
        Some(third) => println!("The third: {}", third),
        None => println!("There is no third element."),
    }

    println!("The third: {}", third);
    let two = &lv[1];
    println!("The two: {}", two);
    // lv.push(9); error
    // println!("The two: {}", two);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.22),
    ];
}