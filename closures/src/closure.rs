use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    // * 注意这个 let 语句意味着 expensive_closure 包含一个匿名函数的 定义，不是调用匿名函数
    //  * 的 返回值。回忆一下使用闭包的原因是我们需要在一个位置定义代码，储存代码，并在之后的位
    //  * 置实际调用它；期望调用的代码现在储存在 expensive_closure 中。
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

// 可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，
// 并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。
// 这种模式被称 memoization 或 lazy evaluation （惰性求值）
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
// - FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。
// 为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的
// Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
// - FnMut 获取可变的借用值所以可以改变其环境
// - Fn 从其环境获取不可变的借用值
// - 大部分需要指定一个 Fn 系列 trait bound 的时候，可以从 Fn 开始，而编译器会根据闭包
// 体中的情况告诉你是否需要 FnMut 或 FnOnce。

// Cacher 结构体的字段是私有的，因为我们希望 Cacher 管理这些值而不是任由调用代码潜在
// 的直接改变他们。
// Cacher::new 函数获取一个泛型参数 T，它定义于 impl 块上下文中并与 Cacher 结构体有
// 着相同的 trait bound。Cacher::new 返回一个在 calculation 字段中存放了指定闭包和
// 在 value 字段中存放了 None 值的 Cacher 实例，因为我们还未执行闭包。
// 当调用代码需要闭包的执行结果时，不同于直接调用闭包，它会调用 value 方法。这个方法
// 会检查 self.value 是否已经有了一个 Some 的结果值；如果有，它返回 Some 中的值并不
// 会再次执行闭包。
// 如果 self.value 是 None，则会调用 self.calculation 中储存的闭包，将结果保存到
// self.value 以便将来使用，并同时返回结果值。
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn closure() {
    let simulated_user_specified_value = 27;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    println!("{}\n{}\n{}\n{}", add_one_v1(0), add_one_v2(1), add_one_v3(2), add_one_v4(3));

    let a = vec![1, 2, 3];
    let equal_to_a = move |z| z == a;
    let b = vec![1, 2, 3];
    assert!(equal_to_a(b));
}