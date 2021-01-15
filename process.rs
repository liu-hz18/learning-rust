#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 进程
// process::Output 结构体表示已结束的子进程（child process）的输出，
// 而 process::Command 结构体是一个进程创建者（process builder）。
// 管道
// std::Child 结构体代表了一个正在运行的子进程，它暴露了 stdin（标准 输入），stdout（标准输出） 和 stderr（标准错误） 句柄，从而可以通过管道与 所代表的进程交互。
use std::error::Error;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::env;

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

// 程序参数
// let args: Vec<String> = env::args().collect();

fn main() {
    let args: Vec<String> = env::args().collect();
    // 第一个参数是调用本程序的路径
    println!("My path is {}.", args[0]);
    // 其余的参数是被传递给程序的命令行参数。
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    // 命令行参数可以通过match解析

    // output: process::Output
    let output = Command::new("rustc").arg("--version")
                          .output()
                          .unwrap_or_else(|e| {
                              panic!("failed to execute process: {}", e)
                          });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n> {}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n> {}", s);
    }

    // pipeline
    // 启动 `wc` 命令
    let process = match Command::new("wc")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why.to_string()),
        Ok(process) => process,
    };
    // 可以把 ChildStdin 和 ChildStdout 看做文件。
    // 将字符串写入 `wc` 的 `stdin`。
    // `stdin` 拥有 `Option<ChildStdin>` 类型
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.to_string()),
        Ok(_) => println!("sent pangram to wc"),
    };
    // 因为 `stdin` 在上面调用后就不再存活，所以它被 `drop` 了，管道也被关闭。
    // `stdout` 字段也拥有 `Option<ChildStdout>` 类型，所以必需解包。
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.to_string()),
        Ok(_) => print!("wc responded with:\n{}", s),
    };

    // 等待进程执行完
    let mut child = match Command::new("sleep").arg("5s").spawn() {
        Err(why) => panic!("couldn't call sleep: {}", why.to_string()),
        Ok(process) => process,
    };
    let _result = child.wait().unwrap();
    println!("reached end of main");
}
