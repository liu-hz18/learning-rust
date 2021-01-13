#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    // if - else
    // Rust 语言中的布尔判断条件 不必用小括号包住，且每个条件后面都跟着一个代码块。
    // 所有分支都必须返回相同的类型。
    let n = -5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let abs_n = if n < 0 { -n } else { n }; // 注意里面的表达式没有分号
    println!("|n| = {}", abs_n);

    // loop
    // Rust 提供了 loop 关键字来实现一个无限循环。
    // 可以使用 break 语句在任何时候退出一个循环，另外可用 continue 跳过循环体的剩 余部分并开始下一轮循环。
    let mut count = 0u32;
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        if (count == 5) { // warn:  remove these parentheses '()'
            println!("now finish at 5");
            break;
        }
    }
    
    // 语法糖！跳出多重循环
    // 在处理嵌套循环的时候可以 break 或 continue 外层循环。
    // 在这类情形中，循环必须用一些 'label（标签）来注明，
    // 并且标签必须传递给 break/continue 语句。
    'outer: loop {
        println!("Enter outer loop");
        'inner: loop {
            println!("Enter inner loop");
            break 'outer; // 写明要跳转的label
        }
        println!("This point will never be reached"); // warn: unreachable statement
    }
    println!("Exited the outer loop");
    // loop 可以带有返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 将被loop返回counter*2 = 20
        }
    };
    assert_eq!(result, 20);

    // while
    let mut n = 1;
    while n < 101 {
        // if n % 15 { // expected `bool`, found integer
        if n % 15 == 0 {
            println!("n = {}", n);
        }
        n += 1;
    }

    // for ... in ... 可以遍历一个 Iterator（迭代器）
    // 创建迭代器的一个最简单的方法是使用区间标记 a..b, [a, b). 步长为 1 
    // a..=b表示两端都包含在内的范围, [a, b]
    for n in 1..100 {
        if n % 15 == 0 {
            println!("n = {}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"]; // T: Vec<&str>
    // iter: 每次迭代中借用集合中的一个元素。这样集合本身不会被改变! 循环之后仍可以使用。
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"), // 传引用
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
    // into_iter 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"), // 传值
            _ => println!("Hello {}", name),
        }
    }
    //println!("names: {:?}", names); // error: value borrowed here after move
    // iter_mut 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!", // 可变对象的引用
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    // match
    // `match` 需要覆盖全部情况。
    let number = 13;
    let a = match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况
        _ => println!("Ain't special"), // `_` 表示不将值绑定到变量
    }; // 返回一个 ()
    // println!("{}", a);
    
    // 可以在match的同时 解构(destructure) tuple
    let pair = (0, -2);
    match pair { // match是有 优先级的，从上到下匹配
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
    }

    // 区分解构和解引用

    // 获得一个 `i32` 类型的引用。`&` 表示取引用。不是取地址！！！
    let reference = &4;
    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（即 `reference` 的类型）
        //    |
        // `&val`（即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 因此可用 `val` 表示被 `reference` 引用的值 4。
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // 如果不想用 `&`，需要在匹配前解引用。
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    let _not_a_reference = 3; // 是一个左值，不是引用
    let ref _is_a_reference = 3; // ref 更改了赋值行为，从而可以对具体值创建引用。
    // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
    let value = 5;
    // 使用 `ref` 关键字来创建引用。
    // 下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
    // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
    // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解引用。
    match value { // 这里的match一定会进入ref r分支，可以视为一种解构
        ref r => println!("Got a reference to a value: {:?}", r), // r是一个引用，由ref创建
    }
    match value {
        ref r => println!("Got a reference to a value: {:?}", *r), // 可以对r解引用
    }
    //match value {
    //    r => println!("Got a reference to a value: {:?}", *r), // error:  type `{integer}` cannot be dereferenced
    //}
    // 类似地使用 `ref mut`。
    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    // 可以加上 match 卫语句（guard） 来过滤分支。
    let pair = (3, 3);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // ^ `if` 条件部分是一个卫语句
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
    // 在match的同时可以绑定 '@' 变量，从而可以在分支中使用该变量的值
    fn age() -> u32 { // 如果函数返回一个值，返回类型必须在箭头 -> 之后指定。除非是单元类型(), 可以省略
        15
    }
    match age() {
        0             => println!("I'm not born yet I guess"),
        // 可以直接 `match` 1 ..= 12，但怎么把岁数打印出来呢？
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结果。
        n             => println!("I'm an old person of age {:?}", n), // matches any value
        _             => println!("None"), // unreachable pattern
    }

    // if-let: 比match更方便的解构/类型判断语法
    let number = Some(7); // type: Option<i32>;
    let letter: Option<i32> = None;
    // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行语句块`{...}`
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else { // 解构失败。切换到失败情形。
        println!("Didn't match a number. Let's go with a letter!");
    }
    // 可以用 if let 匹配任何枚举值
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    // 变量 a 匹配到了 Foo::Bar
    if let Foo::Bar = a { // 注意: 不是Foo::Bar == a， 类型和变量是没有可比性的。只有赋值适合
        println!("a is foobar");
    }
    // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some(i) 类似。
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // while-let: 把别扭的 match 改写得好看一些
    let mut optional = Some(0); // Option<i32>
    // 重复运行这个测试。
    // 当 `let` 将 `optional` 解构成 `Some(i)` 时，就执行语句块（`{}`）。否则就 `break`。
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
    }
    // ^ `if let` 有可选的 `else`/`else if` 分句，
    // 而 `while let` 没有。
}
