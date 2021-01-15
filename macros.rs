#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 宏并不产 生函数调用，而是展开成源码，并和程序的其余部分一起被编译。
// Rust 的宏会展开为抽象语法树（AST，abstract syntax tree），而不是像字符串预处理那样直接替换成代码，这样就不会产生无法预料的优先权 错误。

// 宏是通过 macro_rules! 宏来创建的。
macro_rules! say_hello {
    () => { // `()` 表示此宏不接受任何参数。
        println!("Hello!"); // 此宏将会展开成这个代码块里面的内容。
    };
}

// 宏的参数使用一个美元符号 $ 作为前缀，并使用一个指示符（designator）来 注明类型
// ident: 指示符用于变量名或函数名
// expr: 指示符表示表达式。
// block
// item
// pat: 模式
// path
// stmt: 语句
// tt: token tree, 表示运算符和标记
// ty: type
macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// 借助上述宏来创建名为 `foo` 和 `bar` 的函数。
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // 此宏接受一个 `expr` 类型的表达式
    ($expresstion: expr) => ( // 用圆括号或大括号都可以
        // `stringify!` 把表达式*原样*转换成一个字符串。
        println!("{:?} = {:?}", stringify!($expresstion), $expresstion);
    );
}

// 宏可以重载，从而接受不同的参数组合
// 在这方面，macro_rules! 的作用类似于 匹配（match）代码块
macro_rules! test {
    // 参数不需要使用逗号隔开, 可以使用任意符号，本例中使用分号;
    // 参数可以任意组合！
    ($left: expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}", stringify!($left), stringify!($right), $left && $right);
    }; // 末尾要加分号
    ($left: expr; or $right: expr) => {
        println!("{:?} or {:?} is {:?}", stringify!($left), stringify!($right), $left || $right);
    };
}

// 宏在参数列表中可以使用 + 来表示一个参数可能出现一次或多次，使用 * 来表示该 参数可能出现零次或多次。
macro_rules! find_min {
    ($x: expr) => {
        $x
    };
    // 把模式这样： $(...),+ 包围起来，就可以匹配一个或多个用逗号隔开 的表达式。
    ($x: expr, $($y: expr), +) => {
        std::cmp::min($x, find_min!($($y), +))
    }
}

// 实现新的语言API
// 这其实就是上下文无关文法的翻译模式！定义了语法规则和语义计算
macro_rules! calculate {
    // eval 并不是rust关键字!
    (eval $e:expr) => {
        let val: usize = $e; // 强制类型为整型
        println!("{} = {}", stringify!{$e}, val);
    };
    // 支持可变参数接口
    (eval $e: expr, $(eval $es: expr), +) => {
        calculate!(eval $e);
        calculate!( $(eval $es), + ); // 递归展开
    };
}

fn main() {
    say_hello!(); // 这个调用将会展开成 `println("Hello");`
    
    foo();
    bar();

    print_result!(1u32 + 1);
    // 代码块也是表达式！
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    }); // output: "{ let x = 1u32; x * x + 2 * x - 1 }" = 2

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    println!("{}", find_min!(5u32, 2u32*3, 4u32));

    calculate!(
        eval (1 + 2) * (3 / 4) // 看到了吧，`eval` 可并不是 Rust 的关键字！
    );

    calculate! { // 可变参数的 `calculate!`！
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    };

}
