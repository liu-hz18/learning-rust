// 该属性用于隐藏对未使用代码的警告
#![allow(dead_code)]
/* 块注释 */
// 行注释
/// 文档注释

// 可以在域外声明全局结构体
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// 全局变量是在在所有其他作用域之外声明的。
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn main() {
    // 类型系统

    // 常量
    // const：不可改变的值（通常使用这种）。
    // static：具有 'static 生命周期的，可以是可变的变量（译注：须使用 static mut 关键字）。
    // THRESHOLD = 5; // error: cannot assign to this expression

    // 标量类型
    // i8/i16/i32/i64: 默认i32
    // isize/usize: 和机器相关，是机器最大字长, 也等于指针宽度
    // char: unicode, 4Byte = 32bit
    // (): unit type, 其唯一可能的值就是 () 这个空‘元组’
    // 浮点类型（floating point）： f32、f64 (默认f64)
    // 数组（array）：如 [1, 2, 3]
    // 元组（tuple）：如 (1, true)
    let a_float: f64 = 1.0;  // 常规说明
    let an_integer   = 1_000_000i32; // 后缀说明, 数字可以加下划线区分
    // 变量不能更改
    // a_float = 2.0; // error: cannot assign twice to immutable variable

    // 可变的（mutable）变量，其"值"可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    // 报错！变量的"类型"并不能改变。
    // mutable = true; //error! 
    // 但可以用掩蔽（shadow）（重新声明）来覆盖前面的变量。
    let mutable = true;

    println!("1 - 2 = {}", 1i32 - 2); // 2被推导为i32
    // println!("1 - 2 = {}", 1u32 - 2); // error: attempt to compute `1_u32 - 2_u32`, which would overflow

    // 输出语句与格式化
    // {} 格式化方法是 fmt::Display， 是std::fmt 库下定义的各种宏
    // use `!` to invoke the macro.
    println!("Hello World!");
    // 通常情况下，`{}` 会被任意变量内容所替换。
    println!("I'm {} years old", 20); // 默认 int32
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");
    // 使用命名参数
    println!("{subject} {verb} {object}", subject="the lazydog", verb="the quick", object="jumps over");
    // 格式化输出
    println!("{} of {:b} binary", 20, 20);
    // 右对齐文本
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number=1, width=6);
    // 左补零
    println!("{number:>0width$}", number=1, width=6);

    // 格式化字符串
    let foo:u32 = 3735928559;
    let str1 = format!("{}", foo);
    let str2 = format!("0x{:X}", foo); // 大写Hex
    let str3 = format!("0x{:x}", foo); // 小写Hex
    let str4 = format!("0b{:b}", foo); // Binary
    let str5 = format!("{:e}", foo);   // 科学计数法 1.2e9
    println!("{}\n{}\n{}\n{}\n{}", str1, str2, str3, str4, str5);
    
    // 声明结构体 struct， 包含单个 `i32` ， 命名为 `Structure`
    #[allow(dead_code)]
    struct Structure(i32);
    // 在println!("{}")不能直接打印出来的情况下，使用println!("{:?}")打印复杂类型
    // 所有 std 库类型都天生可以使用 {:?} 来打印
    // 使用 `{:?}` 打印和使用 `{}` 类似。fmt::Display 采用 {} 标记。
    println!("{:?} months in a year.", 12);
    
    // 关于调试， fmt::Debug使得所有类型都可以推导并输出
    // 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
    struct UnPrintable(i32); // warn: struct is never constructed: `UnPrintable`, #[warn(dead_code)]` on by default
    // `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
    #[derive(Debug)]
    struct DebugPrintable(i32);
    println!("Now {:?} will print!", DebugPrintable(3));
    #[derive(Debug)]
    struct DeepDebugPrintable(DebugPrintable);
    println!("Now {:?} will print!", DeepDebugPrintable(DebugPrintable(3)) );

    // rust 也通过 {:#?} 提供了 “美化打印” 的功能
    let name = "Potter";
    let age:u8 = 27; // rust是静态类型语言，可以定义类型
    let potter = Person {name, age};
    println!("{:#?}", potter); // 此时的输出是经过美化的，便于读
    println!("{:?}", potter); // 此时输出只有一行，不适合人读取
    // println!("{}", potter); // 此时报错， `Person<'_>` doesn't implement `std::fmt::Display`
    
    // 自定义Display对结构体的输出格式，并和Debug对比
    // 使用 use 声明的话，就可以不写出名称的完整路径了
    use std::fmt;
    // 或者
    // use std::fmt::{self, Formatter, Display};
    // 带有两个数字的结构体。推导出 `Debug`，以便与 `Display` 的输出进行比较。
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // 实现 `MinMax` 的 `Display`。
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 使用 `self.number` 来表示各个数据。
            // 表达式可以计算并产生一个值，也就是右值；但是声明语句不是表达式，不能作为右值。let a = (let b = 5);是不合法的
            // 不加分号的是一个表达式，加了分号就是一个语句(表达式语句)。
            // 在函数里可以 最后有一个表达式，默认省略return和分号
            // 注意下面语句之后不能带分号，因为需要返回一个fmt::Result类型的对象。
            write!(f, "DIY:({}, {})", self.0, self.1)
            // 等价于
            // return write!(f, "DIY:({}, {})", self.0, self.1);
            // 代码块也是表达式，所以它们在赋值操作中可以充当右值（r-values）。代码块中的最后一条表达式将赋给左值（l-value）。
            // 需要注意的是，如果代码块最后一条表达式结尾处有分号，那 么返回值将变成 ()。
        }
    }

    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    // ? 运算符: statement? 等价于 try!(statement), 含义是
    // 对 statement 进行尝试（try），观察是否出错。若发生错误，返回相应的错误。
    // 否则（没有出错）继续执行后面的语句。
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                // 对每个元素（第一个元素除外）加上逗号。
                // 使用 `?` 或 `try!` 来返回错误。
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }
            write!(f, "]")
        }
    }
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    // 复杂类型
    // 元组
    // 元组可以充当函数的参数和返回值
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // 可以使用 `let` 把一个元组的成员绑定到一些变量
        let (integer, boolean) = pair;
        (boolean, integer)
    }
    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("tuple: {:?}", long_tuple); // 注意println!("tuple: {}", long_tuple); 不行
    
    // 但很长的元组无法打印(元素 > 12个)
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple); // error: cannot be formatted using `{:?}` because it doesn't implement `Debug`
    
    // 创建单元素元组需要一个“额外的逗号”，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));

    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    // 数组
    // 数组（array）是一组拥有相同类型 T 的对象的集合，在内存中是连续存储的。大小在编译期确定
    // 定长数组（类型标记i32是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [1; 500]; // 批量初始化为1
    println!("first element of the array (len = {}): {}", xs.len(), xs[0]);
    // 数组是在栈中分配的
    println!("array occupies {} bytes", std::mem::size_of_val(&xs));
    // 越界的下标会引发致命错误（panic）,在编译期确定！
    // println!("{}", xs[5]); //error: index out of bounds: the length is 5 but the index is 5
    
    // 切片
    // 切片（slice）类型和数组类似，但其大小在编译时是不确定的。
    // 相反，切片是一个双字 对象（two-word object），第一个字是一个指向数据的指针，第二个字是切片的长度。
    // slice 可以用来借用array的一部分。slice 的类型标记为 &[T]。
    // x..y 表示 [x, y) 的数学含义。.. 两边可以没有运算数：
    // ..y 等价于 0..y
    // x.. 等价于位置 x 到数据结束
    // .. 等价于位置 0 到结束
    // 切片结果必须是引用类型, 开发者必须自己明示这一点: &array[a..b]
    let xs_slice = &xs[1..3]; // 长度为2， 范围[1, 3)
    println!("{:?} (len = {})", xs_slice, xs_slice.len());
    
    // 字符串
    // str & String
    // str是字符串切片(String slice), 常常以引用的形式出现（&str）
    // 凡是用双引号包括的 字符串常量 整体的类型 都是 &str
    let s = "hello";
    // String 类型是 Rust 标准公共库提供的一种数据类型，它的功能更完善——它支持字符串的追加、清空等实用的操作。
    // String 和 str 都支持切片，切片的结果是 &str 类型的数据。
    let a_string = String::from("hello");
    let a_str = &a_string[..];

    // 结构体
    
    // 元组结构体类型
    struct Pair(i32, f32);
    // 单元结构体，不带字段，在泛型中很有用。
    struct Nil;
    // 带有两个字段（field）的结构体
    struct Point {
        x: f32,
        y: f32, // 最后可以有逗号
    }
    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };
    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);
    // 使用结构体更新语法创建新的 point，这样可以用到之前的 point 的字段
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", new_point.x, new_point.y);
    // 使用 `let` 绑定来解构 point 到 my_x, my_y
    let Point { x: my_x, y: my_y } = point;
    println!("{}, {}", my_x, my_y);

    // enum： 定义一个枚举类型（enumeration）
    // 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
    // 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
    // `Paste(String)`。各个取值不同，互相独立。
    enum WebEvent {
        // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
        PageLoad,
        PageUnload,
        // 或者一个元组结构体
        KeyPress(char),
        Paste(String),
        // 或者一个普通的结构体。
        Click { x: i64, y: i64 },
    } // 函数或类型声明不用加分号

    // 此函数将一个 `WebEvent` enum 作为参数，无返回值。
    fn inspect(event: WebEvent) {
        // match
        // 枚举的目的是对某一类事物的分类, 往往枚举类最终都会被分支结构 match 处理
        // 对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事. 例外情况用下划线 _ 表示
        match event {
            WebEvent::PageLoad => println!("page loaded"), // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
            WebEvent::PageUnload => println!("page unloaded"),
            // 从 `enum` 里解构出 `c`。
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c), // 元组结构体
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // 把 `Click` 解构给 `x` and `y`。
            WebEvent::Click { x, y } => { // 普通结构体 C-style
                println!("clicked at x={}, y={}.", x, y);
            },
            _ => {
                println!("Oops!");
            },
        }
    }
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // 类型别名(type alias)
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add, // 单元结构体, 并不含有值，可以作为一个类型占位符
        Sub,
    }
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    // 最常见的情况就是在 impl 块中使用 Self 别名。
    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y, // Self默认为本类别名
                Self::Sub => x - y,
            }
        }
    }
    let add_operation = Operations::Add; // enum的实例化，不是结构体的实例化(当然可以看做结构体)
    let sub_operation = Operations::Sub;
    println!("{}", add_operation.run(1, 2));
    println!("{}", sub_operation.run(1, 2));

    // enum 也可以像 C 语言风格的枚举类型那样使用
    // 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
    enum Number {
        Zero,
        One,
        Two,
    }
    println!("zero is {}", Number::Zero as i32); // 0
    println!("one is {}", Number::One as i32); // 1

    // 拥有显式辨别值（explicit discriminator）的 enum
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    
    
}
