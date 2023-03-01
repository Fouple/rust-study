use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Black"), String::from("White")];
    let initial_scores = vec![100, 99];
    let mut n_scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // hash map 所有权
    // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。对于像 String
    // 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let num_name = 32;
    let num_value = 64;

    let mut map1 = HashMap::new();
    map1.insert(field_name, field_value);
    // println!("{}, {}", field_name, field_value); // 这里 field_name 和 field_value 不再有效，
    let mut map2 = HashMap::new();
    map2.insert(num_name, num_value);
    println!("{}, {}", num_name, num_value);

    // 访问 hash map 中的值
    match scores.get(&String::from("Blue")) {
        Some(score) => println!("score: {}", score),
        None => println!("nothing"),
    }
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("{:?}", scores);

    // 更新 hash map
    // 覆盖
    let mut test_scores = HashMap::new();
    test_scores.insert(String::from("Blue"), 88);
    test_scores.insert(String::from("Blue"), 77);
    println!("{:?}", test_scores);

    // 只在键没有对应值时插入
    test_scores.entry(String::from("Yellow")).or_insert(70);
    test_scores.entry(String::from("Blue")).or_insert(40);
    println!("{:?}", test_scores);

    // 根据旧值更新一个值
    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    
    
}
