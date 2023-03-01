// `block_on`会阻塞当前线程直到指定的`Future`执行完成，这种阻塞当前线程以等待任务完成的方式较为
// 简单、粗暴，好在其它运行时的执行器(executor)会提供更加复杂的行为，例如将多个`future`调度到同
// 一个线程上执行。
use futures::executor::block_on;

async fn hello_world() {
    hello_cat().await;
    println!("hello, world !");
}

async fn hello_cat() {
    println!("hello, kitty !");
}

fn hello() {
    let future = hello_world();
    block_on(future);
}
