#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

// 返回变量类型
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    // Rust 通过静态类型确保类型安全。变量绑定可以在声明时说明类型，
    // 不过在多数情况下， 编译器能够从上下文推导出变量的类型，从而大大减少了类型说明的工作。
    // 变量绑定默认是不可变的（immutable），但加上 mut 修饰语后变量就可以改变。
    let unit = ();
    let mut mutable_binding = 1;
    mutable_binding += 1;
    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
    let noisy_unused_variable = 2u32;
    let _unused_variable = 3u32;
    // 重新声明mutable_binding，引发了变量掩蔽(variable shadowing)
    let mutable_binding = 2;
    //mutable_binding += 1; // error! 此时变量不能再改变
    
    // 允许声明后赋值，但是未赋值的使用会报错
    let a_binding; // 声明一个变量绑定
    //println!("a non-inited binding: {}", a_binding); // error: use of possibly-uninitialized `a_binding`
    a_binding = 1; // 初始化一个绑定
    println!("a inited binding: {}", a_binding);
    
    // 类型转换
    let decimal = 65.4321_f32; // 右边是一个 数值字面量
    // Rust 不提供原生类型之间的隐式类型转换（coercion）
    //let integer: u8 = decimal; // error: expected `u8`, found `f32`
    // 但可以使用 as 关键字进行显式类型转换（casting）, number as type 是一个右值
    let integer: u8 = decimal as u8;
    let character: char = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1)
    // 直到值位于新类型 T 的范围内。
    // 也就是常识下 int 和 uint 的溢出处理
    // 在 #![allow(overflowing_literals)] 下是没有error的, 否则得到error而不是warning
    // 1000 - 256 - 256 - 256 = 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是 -24
    // std::mem::size_of_val(&literal), pass by reference!
    println!(" 232 as a i8 is : {}, {}, {}", 232 as i8, std::mem::size_of_val(&232), std::mem::size_of_val(&(232 as i8))); // the literal `232` does not fit into the type `i8` whose range is `-128..=127`
    // Rust 的类型推断引擎是很聪明的，它不只是在初始化时看看右值（r-value）的 类型而已，
    // 它还会考察变量之后会怎样使用，借此推断类型。
    // 创建一个空向量（vector，即不定长的，可以增长的数组）。
    let mut vec = Vec::new();
    // 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）
    let elem = 5u8;
    vec.push(elem); // 现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）
    println!("{:?}", vec); // 如果注释掉上面这行，就无法推导类型，报错:cannot infer type for type parameter `T`

    // 类型别名: 避免写出冗长的模板化代码
    type NanoSecond = u64; // rust 建议类型使用驼峰命名法，而不是下划线
    type u64_t = u64; // warning: convert the identifier to upper camel case: `U64T`

    let a_number: u64_t = 10 as u64;
    
    // trait: from/into
    // trait 是对未知类型 Self 定义的方法集。可以看做 C++抽象类或JAVA接口
    // From trait 允许一种类型定义 “怎么根据另一种类型生成自己”
    let my_str = "hello";
    let my_string = String::from(my_str); // str -> String
    // 自定义类型转换
    //use std::convert::From;
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number { // 实现 From Trait
        fn from(item: i32) -> Self {
            Number {value: item}
        }
    }
    let num = Number::from(30); // construct a Number 'from' integer
    println!("My number is: {:?}", num); // not a function, a macro
    // 如果你为你的类型实现了 From，那么同时你也就免费获得了 Into
    // 使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。
    let a_int_number = 5;
    let num: Number = a_int_number.into(); // convert a integer 'into' Number
    println!("My number is: {:?}", num);
    // 类似于 From 和 Into，TryFrom 和 TryInto 是 类型转换的通用 trait。
    // 不同于 From/Into 的是，TryFrom 和 TryInto trait 用于易出错的转换

    // 有关字符串的类型转换
    let parsed_int: i32 = "5".parse().unwrap(); // 必须在某一侧规定类型
    let turbo_parsed_int = "10".parse::<u32>().unwrap(); // 指定类型
    let parsed_f32: f64 = "3.2".parse().unwrap();
    println!("{}, {}, {}", parsed_int, turbo_parsed_int, parsed_f32);

    let y = {
        let x = 5u32;
        let x_squ = x * x;
        let x_cube = x_squ * x;
        x_cube + x_squ + x // 没有分号，是一个表达式（右值），作为返回值
    };
    println!("y = {}", y);
    
    // 不安全操作:
    // 裸指针: rust依然有这种类型
    // 原始指针（raw pointer，裸指针）* 和引用 &T 有类似的功能，但引用总是安全的，因为借用检查器保证了它指向一个有效的数据。
    // 解引用一个裸指针只能通过 不安全 代码块执行。
    let raw_p: *const u32 = &10;
    //assert!(*raw_p == 10); // error: dereference of raw pointer
    unsafe {
        assert!(*raw_p == 10);
    }
    // 一些函数可以声明为不安全的（unsafe），这意味着在使用它时保证正确性不再是编译器 的责任，而是程序员的。
}
