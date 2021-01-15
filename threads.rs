#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 通过 spawn 函数提供了创建本地操作系统（native OS）线程的机制
// 该函数的参数是一个通过值捕获变量的闭包 moving closure
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::Duration; 

static NTHREADS: i32 = 10;

// 标准库提供了开箱即用的线程类型，把它和 Rust 的所有权概念与别名规则结合 起来，可以自动地避免数据竞争（data race）。
// 当某状态对某线程是可见的，别名规则（即一个可变引用 XOR 一些只读引用。注：XOR 是异或的意思，即「二者仅居其一」）就自动地避免了别的线程对它的操作。
// 当需要同步 处理时，请使用 Mutex 或 Channel 这样的同步类型。
// 虽然我们在线程之间传递了引用，但 Rust 理解我们是在传递只读的引用，因此 不会发生数据竞争等不安全的事情。

// Rust 为线程之间的通信提供了异步的通道（channel）。
// 通道允许两个端点之间信息的 单向流动：Sender（发送端） 和 Receiver（接收端）。

// 主线程
fn main() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        // 启动（spin up）另一个线程
        // 因为我们把数据块 move 到了线程中，Rust 会保证数据存活至线程退出，因此不会产生悬挂指针。
        // spawn() 返回新线程的句柄（handle），我们必须拥有句柄，才能获取线程的返回值。
        // 
        children.push(thread::spawn(move | | {
            // println! 会锁住标准输出，这样各线程打印的内容不会交错在一起
            println!("this is thread number {}", i);
            thread::sleep(Duration::from_millis(1000)); // 线程休眠时会自动让出 cpu
        }));
    }
    // 这些线程由操作系统调度（schedule）
    for child in children {
        // 把子线程加入主线程等待队列
        let _ = child.join(); // join 会返回这个线程的返回值
    }

    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
    let mut children = vec![];
    // map
    let chunked_data = data.split_whitespace(); // 每段都是完整数据的一个引用（&str）
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);
        children.push(std::thread::spawn(move | | -> u32 {
            // expect: expect(self,msg:&str):T
            // 如果 self 是 Ok 或 Some 则返回包含的值。否则调用panic!() 输出"自定义的错误" msg 并退出
            let result = data_segment
                         .chars()
                         .map(|c| c.to_digit(10).expect("should be a digit"))
                         .sum();
            println!("processed segment {}, result = {}", i, result);
            result // Rust 是一种 “表达式语言”, 每个代码块中最后求值的表达式就是代码块的值。
        }));
    }
    // reduce
    let mut intermediate_sums = vec![];
    for child in children {
        intermediate_sums.push(child.join().unwrap());
    }
    let final_result: u32 = intermediate_sums.iter().sum();
    println!("Final sum result: {}", final_result);

    // 通道有两个端点：`Sender<T>` 和 `Receiver<T>`，其中 `T` 是要发送的消息的类型（类型标注是可选的）
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for id in 0..NTHREADS {
        // sender 端可被复制
        let thread_tx = tx.clone();
        // 每个线程都将通过通道来发送它的 id
        std::thread::spawn(move || {
            // 被创建的线程取得 `thread_tx` 的所有权, 每个线程都把消息放在通道的消息队列中
            thread_tx.send(id).unwrap();
            // 发送是一个非阻塞（non-blocking）操作，线程将在发送完消息后会立即继续进行
            println!("thread {} finished", id);
        });
    }
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS { // _ 是 reserved identifier
        // `recv` 方法从通道中拿到一个消息, 若无可用消息的话，`recv` 将阻止当前线程
        ids.push(rx.recv());
    }
    // 显示消息被发送的次序
    println!("{:?}", ids);
}
