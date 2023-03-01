/// #错误处理指导原则
/// 
/// 在当有可能会导致有害状态的情况下建议使用 panic! —— 在这里，有害状态是指当一些假设、
/// 保证、协议或不可变性被打破的状态，例如无效的值、自相矛盾的值或者被传递了不存在的值
///  —— 外加如下几种情况：
///
/// 有害状态是非预期的行为，与偶尔会发生的行为相对，比如用户输入了错误格式的数据。
/// 在此之后代码的运行依赖于不处于这种有害状态，而不是在每一步都检查是否有问题。
/// 没有可行的手段来将有害状态信息编码进所使用的类型中的情况。
/// 
/// 当错误预期会出现时，返回 Result 仍要比调用 panic! 更为合适。这样的例子包括解析器
/// 接收到格式错误的数据，或者 HTTP 请求返回了一个表明触发了限流的状态。在这些例子中，
/// 应该通过返回 Result 来表明失败预期是可能的，这样将有害状态向上传播，调用者就可以
/// 决定该如何处理这个问题。


/// #创建自定义类型进行有效性验证

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}