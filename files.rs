#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 路径 
// Path 结构体代表了底层文件系统的文件路径。
// Path 分为两种：posix::Path，针对 类 UNIX 系统；以及 windows::Path，针对 Windows。
// prelude 会选择并输出符合平台类型 的 Path 种类。
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

static LOREM_IPSUM: &'static str =
"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

// File 结构体表示一个被打开的文件（它包裹了一个文件描述符），并赋予了对所表示的 文件的读写能力。
// File 的所有方法都 返回 io::Result<T> 类型，它是 Result<T, io::Error> 的别名。
// File 拥有资源，即文件描述符（file descriptor），它会在自身被 drop 时关闭文件。
// 文件相关的错误一般是运行时的，而不是编译期的。

// 输出包裹在 Result 中以允许匹配错误，将迭代器返回给文件行的读取器（Reader）。
// File::open 需要一个泛型 AsRef<Path>。这正是 read_lines() 期望的输入。
// 这个过程比在内存中创建 String 更有效，特别是处理更大的文件。
type FileLineReader = std::io::Lines<std::io::BufReader<File>>;
fn read_lines<P>(filename: P) -> io::Result<FileLineReader>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(BufReader::new(file).lines())
}

// 返回变量类型
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    // 从 `&'static str` 创建一个 `Path`
    let path = Path::new("."); // Debug Trait
    let display = path.display(); // Display Trait
    let new_path = path.join("a").join("b");
    println!("{}", display);
    println!("{:?}", new_path);
    
    // 将路径转换成一个字符串切片
    // 注意 Path 在内部并不是用 UTF-8 字符串表示的，而是存储为若干字节（Vec<u8>）的 vector。
    // 因此，将 Path 转化成 &str 并非零开销的（free），且可能失败（因此它 返回一个 Option）。
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }

    // create 静态方法以只写模式（write-only mode）打开一个文件。若文件已经存在，则 旧内容将被销毁。否则，将创建一个新文件。
    let path = Path::new("./lorem_ipsum.txt");
    let display = path.display();


    // 以只写模式打开文件，返回 `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
    // write_all() 方法并不会在写入结束后自动写入换行符 \n
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
        Ok(_)    => println!("successfully wrote to {}", display),
    };


    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    // 读取文件内容到一个字符串，返回 `io::Result<usize>`, 表示内容的长度
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why)  => panic!("couldn't read {}: {}", display, why.to_string()),
        Ok(strlen)=> print!("{} contains:\n{} {}", display, s, strlen),
    };

    // 按行读取
    // 方法 lines() 在文件的行上返回一个迭代器。 io::Lines<io::BufReader<File>>
    if let Ok(lines) = read_lines(&"./lorem_ipsum.txt") {
        // 使用迭代器，返回一个（可选）字符串
        //println!("{}", type_of(&lines)); // lines: Lines<BufReader<File>>
        for line in lines { // line: io::Result<String>
            //println!("{}", type_of(&line));
            if let Ok(content) = line { // content: String
                //println!("{}", type_of(&content));
                println!("{}", content);
            }
        }
    } else {
        println!("File not exists. ");
    }


    // 追加内容，可以实现行写入
    let mut file = match OpenOptions::new().append(true).open(&"./lorem_ipsum.txt") {
        Err(why) => panic!("couldn't open(a+) {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    match file.write_all("write a new line\n".as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.to_string()),
        Ok(_)    => println!("successfully wrote to {}", display),
    };


    // `file` 离开作用域，并且 `./lorem_ipsum.txt` 文件将被关闭。
}
