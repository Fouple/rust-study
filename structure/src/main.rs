/**
 * 结构体让你可以创建出在你的领域中有意义的自定义类型。通过结构体，我们可以将相关联
 * 的数据片段联系起来并命名它们，这样可以使得代码更加清晰。在 impl 块中，你可以定义
 * 与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的实例所具有的行为。
 * 但结构体并不是创建自定义类型的唯一方法：枚举。
 */

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/* 元组结构体 tuple structs */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* 类单元结构体 unit-like structs */
struct AlwaysEqual;

/* 结构体的所有权 test */
// struct UserTest {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

/*
 * 方法（method）与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，
 * 同时包含在某处调用该方法时会执行的代码。不过方法与函数是不同的，因为它们在结构
 * 体的上下文中被定义（或者是枚举或 trait 对象的上下文，将分别在第六章和第十七章
 * 讲解），并且它们第一个参数总是 self，它代表调用该方法的结构体实例。
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/** 
 * 所有在 impl 块中定义的函数被称为 关联函数（associated functions），因为它们与 
 * impl 后面命名的类型相关。 
 * 方法：以 self 为第一参数的关联函数
 */
impl Rectangle {
    // 方法、关联函数
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法、关联函数
    fn addHeightToArea(&mut self, length: u32) -> u32 {
        self.height += length;
        self.area()
    }

    // 关联函数
    // :: 语法用于关联函数和模块创建的命名空间
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// 每个结构体都允许拥有多个 impl 块。
impl Rectangle {
    // 方法、关联函数
    fn canHold(&self, other: &Rectangle) -> bool {
        let mut isCan = false;
        if self.width > other.width && self.height > other.height {
            isCan = true;
        } else if self.height > other.width && self.width > other.height {
            isCan = true;
        }
        isCan
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("lifupei@wondergroup.com"),
        username: String::from("lifupei"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("1219982329@qq.com");
    println!("{}: {}", user1.username, user1.email);
    let user2 = build_user(String::from("fouple@gmail.com"), String::from("fouple"));
    println!("{}: {}", user2.username, user2.email);
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // 请注意，结构更新语法就像带有 = 的赋值，因为它移动了数据，
    // println!("{}", user1.username); error!!
    println!("{}: {}", user1.active, user1.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // let userTest1 = UserTest {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };

    let scale = 2;
    let mut rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    println!("area: {}", rect1.addHeightToArea(50));

    let sq = Rectangle::square(3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
