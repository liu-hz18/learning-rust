#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 介绍泛型与其约束功能
use std::fmt::{Debug, Display};

// 约束
// 用 Display 来约束 T，也就是说 T 必须实现 Display。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

// 可以使用空约束来约定 某类型执行某方法的权限
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 这些函数只对实现了相应的 trait 的类型有效。事实上这些 trait 内部是空的，但这没有关系。
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

// 多重约束使用 +
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}
// 类型之间使用 , 隔开。
fn compare_types<T: Debug, U: Display>(t: &T, u: &U) {
    println!("{:?}, {}", t, u);
}

// 也可以使用where改写:
fn compare_types_where<T, U>(t: &T, u: &U) where
    T: Debug,
    U: Debug + Display {
    println!("{:?}, {}", t, u);
}

// 定义接口
trait PrintInOption {
    fn print_in_option(self);
}

// where子句比泛型类型说明有更强的能力
// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
impl<T> PrintInOption for T where
    // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
    // 否则我们会给出错误的约束。
    Option<T>: Debug {
        fn print_in_option(self) {
            println!("{:?}", Some(self)); // 封装成 Option<T>
        }
    }


// 关联类型：在Trait中定义泛型类型，从而减少使用泛型时的代码量
struct Container(i32, i32); // 元组结构体

// 这个 trait 检查给定的 2 个项是否储存于容器中. 并且能够获得容器的第一个或最后一个值。
trait Contains {
    // 在这里定义可以被方法使用的泛型类型。
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool; // _作为占位参数
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}
// 'defaults' for type parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions.
// 不使用关联类型:
// trait Contains<A = i32, B> { // 表示A默认为i32
//     fn contains(&self, _: &A, _: &B) -> bool; // 显式地要求 `A` 和 `B`
//     fn first(&self) -> i32; // 未显式地要求 `A` 或 `B`
//     fn last(&self) -> i32;  // 未显式地要求 `A` 或 `B`
// }

impl Contains for Container {
    // 指出 `A` 和 `B` 是什么类型。如果 `input`（输入）类型
    // 为 `Container(i32, i32)`，那么 `output`（输出）类型
    // 会被确定为 `i32` 和 `i32`。
    type A = i32;
    type B = i32;

    // `&Self::A` 和 `&Self::B` 在这里也是合法的类型。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // 得到第一个数字。
    fn first(&self) -> i32 { self.0 }

    // 得到最后一个数字。
    fn last(&self) -> i32 { self.1 }
}

// 不使用关联类型:
// fn difference<A, B, C>(container: &C) -> i32 where
//     C: Contains<A, B> { 
//     container.last() - container.first()
// }

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;
    // 由于约束，`red()` 不能作用于 blue_jay，反过来也一样。
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey)); // error: the trait `Red` is not implemented for `Turkey`

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];
    compare_prints(&string);
    //compare_prints(&array); // error: `[{integer}; 3]` cannot be formatted with the default formatter

    compare_types(&array, &string);
    compare_types_where(&array, &string);

    // vec!宏是一个 有 Debug Trait, 同时 Option<vec!> 也有Debug Trait
    let vec = vec![1, 2, 3];
    vec.print_in_option();

    // 关联类型示例
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);
    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("The difference is: {}", difference(&container));
}
