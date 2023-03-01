// trait是一个定义泛型行为的方法。trait 可以与泛型结合来将泛型限制为拥有特定行为的类型，
// 而不是任意类型

// Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。(运行时)
// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是
// 一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

struct Point<T> {
    x: T,
    y: T,
}

// 因为再次声明了泛型，我们可以为泛型参数选择一个与结构体定义中声明的泛型参数所不同
// 的名称，不过依照惯例使用了相同的名称。
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，而其他 T
// 不是 f32 类型的 Point<T> 实例则没有定义此方法。
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
struct Pointy<X, Y> {
    x: X,
    y: Y,
}

impl<X1, Y1> Pointy<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Pointy<X2, Y2>) -> Pointy<X1, Y2> {
        Pointy {
            x: self.x,
            y: other.y,
        }
    }
}

fn generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("p.x = {}", float.x());

    let p1 = Pointy {x: 5, y: 10.4};
    let p2 = Pointy {x: "Hello", y: 'c'};
    let p3 = p2.mixup(p1);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
