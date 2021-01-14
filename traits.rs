#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 运算符重载
use std::ops;
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;
// 下面的代码块实现了 `Foo + Bar = FooBar` 这样的运算。
impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}
// 下面的代码块实现了 `Bar + Foo = BarFoo` 这样的运算。
// 通过颠倒类型，我们实现了不服从交换律的加法。
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

// Drop
// Drop trait 只有一个方法：drop，当对象离开作用域时会自动调用该方法。
// Drop trait 的主要作用是释放实现者的实例拥有的资源。
// Box，Vec，String，File，以及 Process 是一些实现了 Drop trait 来释放 资源的类型。
struct Droppable {
    name: &'static str,
}
// 这个简单的 `drop` 实现添加了打印到控制台的功能。也可以手动调用 drop(_a);
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

// 迭代器 Iterator Trait
// 这个 trait 只需定义一个返回 next（下一个）元素的方法，这可手动在 impl 代码块中定义
// for 结构会使用 .into_iterator() 方法将一些集合类型 转换为迭代器。
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    // 我们在这里使用 `.curr` 和 `.next` 来定义数列（sequence）。
    // 返回类型为 `Option<T>`：
    //     * 当 `Iterator` 结束时，返回 `None`。
    //     * 其他情况，返回被 `Some` 包裹（wrap）的下一个值。
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        // 既然斐波那契数列不存在终点，那么 `Iterator` 将不可能
        // 返回 `None`，而总是返回 `Some`。
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 } // 返回一个斐波那契数列生成器
}

// Clone Trait
// 通常，我们可以使用由 Clone trait 定义的 .clone() 方法。
// 一个包含资源的结构体，它实现了 `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);


fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
    
    { // A
        let _a = Droppable { name: "a" };
        println!("Exiting block A");
    }
    println!("Just exited block A");
    
    // `for` 遍历 `Iterator` 直到返回 `None`，
    // 并且每个 `Some` 值都被解包（unwrap），然后绑定给一个变量（这里是 `i`）。       println!("Iterate through 0..3 using `for`");
    for i in 0..3 { // `0..3` 是一个 `Iterator`，会产生：0、1 和 2。
        println!("> {}", i);
    }

    // `take(n)` 方法提取 `Iterator` 的前 `n` 项。
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // `skip(n)` 方法移除前 `n` 项，从而缩短了 `Iterator` 。
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
    // `iter` 方法对数组/slice 产生一个 `Iterator`。
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }

    // 实例化 `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);
    // 将 `pair` 绑定到 `moved_pair`，移动（move）了资源
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);
    // 报错！`pair` 已失去了它的资源。
    //println!("original: {:?}", pair);
    // 将 `moved_pair`（包括其资源）克隆到 `cloned_pair`。
    let cloned_pair = moved_pair.clone();
    drop(moved_pair); // 使用 std::mem::drop 来销毁原始的 pair。
    println!("clone: {:?}", cloned_pair);// 由 .clone() 得来的结果仍然可用！

}
