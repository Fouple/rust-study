use std::io;

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let spaces = "        ";
    println!("spaces:{spaces}");
    let spaces = spaces.len();
    println!("spaces:{spaces}");

    let isize_max: isize = isize::MAX;

    println!("isize_max is {isize_max}");

    let char_max = char::MAX;
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("char max: {char_max},c: {c}, z: {z}, 😻: {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup: {x}, {y}, {z}");
    let tup_first = tup.0;
    println!("tup.0: {tup_first}");

    let mut tup_var: (i128, f32, char) = (0, 0.0, '\0');
    tup_var.0 = 32;
    tup_var.1 = 64.01;
    tup_var.2 = '💢';
    let (tap0, tap1, tap2) = tup_var;
    println!("tup var: {tap0}, {tap1}, {tap2}");

    let tup_unit: () = ();
    let _blank = tup_unit;

    let a: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let b = [3; 10];
    println!("a[0]: {} \n b[5]: {}", a[0], b[5]);
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
