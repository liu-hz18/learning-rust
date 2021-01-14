// 定义若干属性attribute， #[attribute(value)] 
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 定义数据
struct Point {
    x: f64,
    y: f64,
}
// 定义Point的方法
impl Point {
    // static method, 不需要被实例调用，作为constructor
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    // 实例方法 instance method, `&self` 是 `self: &Self` 的语法糖（sugar），其中 `Self` 是方法调用者的类型, 即Point
    fn length(&self) -> f64 {
        let Point{x: x, y: y} = self; // 解构Self到x, y
        (x * x + y * y).sqrt()
    }
    // 这个方法要求调用者是可变的
    // `&mut self` 为 `self: &mut Self` 的语法糖
    fn move_to(&mut self, x: f64, y: f64) { // rust建议函数名为 小写单词+下划线
        self.x = x;
        self.y = y;
    }

}


fn main() {
    // 函数: fn name -> return_type {  }
    // 方法: 依附于对象，在 impl 代码块中定义。
    let mut p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point::origin();
    println!("length = {}", p1.length());
    p1.move_to(2.0, 2.0);
    println!("length = {}", p1.length());

    // lambda表达式 (闭包) |val: type| -> type { (return) ... };
    // 调用闭包时，输入和返回类型两者都可以自动推导，而输入变量名必须指明。
    // 函数:
    fn function (i: i32) -> i32 { i + 1 }
    // lambda:
    let closure_annotated = |i: i32| -> i32 { i + 1 }; // 通过引用：&T，获取了不可变的引用 (Fn)
    let closure_inferred = |i| i + 1;
    let closure_const_one = || 1;
    // 闭包表达式产生的类型就是 “闭包类型”，不属于引用类型，而且确实无法对上面两个`closure_xxx` 变量解引用。
    let i = 1;
    // 调用函数和闭包。
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    println!("closure returning one: {}", closure_const_one());

    // 捕获capture变量
    let mut count = 1;
    // 闭包中维护了可变变量 &mut count(可变引用). 所以闭包是可变的，所以需要mut关键字
    let mut increment = | | { // 通过可变引用：&mut T，可以改变 变量 (FnMut)
        count += 1; // 捕获并修改count
        println!("count = {}", count);
    };
    increment(); // 借用counter
    increment();

    use std::mem;
    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);
    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的“值”。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || { // 通过值：T，拿到了变量的所有权而非借用 (FnOnce)
        println!("`movable`: {:?}", movable);
        mem::drop(movable); // 消耗了该变量, closure cannot be invoked more than once because it moves the variable `movable` out of its environment
        // this value implements `FnOnce`, which causes it to be moved when called
    };
    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume(); // `consume` moved due to this call
    //consume(); // error: value used here after move


    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3]; // move occurs because `haystack` has type `Vec<i32>`, which does not implement the `Copy` trait
    // 在竖线 | 之前使用 move 会强制闭包取得被捕获变量的所有权（传值）
    let contains = move |needle| haystack.contains(needle); // haystack被移动进闭包中, value moved into closure here, due to use in closure
    println!("{}", contains(&1));
    println!("{}", contains(&4)); // haystack没有被消耗，只是在编译期移入到了闭包中

    //println!("There're {} elements in vec", haystack.len()); // value borrowed here after move
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。
    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。

    // 闭包类型: Fn, FnMut, FnOnce
    // Fn、FnMut 和 FnOnce 这些 trait 明确了闭包如何从周围的作用域中 "捕获" 变量。
    // 例如用一个类型说明为 FnOnce 的闭包作为参数。这说明闭包可能采取 &T，&mut T 或 T 中的一种捕获方式，但编译器最终是根据所捕获变量在闭包里的使用情况决定捕获方式。
    // 这是因为如果能以 移动 的方式捕获变量，则闭包也有能力 使用其他方式借用变量。
    // 如果参数的类型说明是 Fn，那么不允许该闭包通过 &mut T 或 T 捕获变量。
    fn apply<F>(f: F) where F: FnOnce() { // 闭包作为函数参数， F必须是泛型的
        f();
    }
    fn call_me<F: FnMut()>(mut f: F) { // 也可以这么定义泛型
        f();
    }
    // 当闭包被定义，编译器会隐式地创建一个匿名类型的结构体，用以储存闭包捕获的 变量，
    // 同时为这个未知类型的结构体实现函数功能，通过 Fn、FnMut 或 FnOnce 三种 trait 中的一种。
    // 事实上，指明为该 结构体实现的是 Fn、FnMut、或 FnOnce 中的哪种 trait，对于约束该结构体的 类型而言就已经足够了。
    // 任何满足该闭包的 trait 约束的"函数（Function）"也可以作为其参数

    // 闭包作为函数返回值: 通过 impl Trait 实现
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        // 必须使用 move 关键字，它表明所有的捕获都是通过值进行的。这是必须的，
        // 因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。
        move || println!("This is a: {}", text)
    }

    // std中的例子
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32`。（出于数组不可变的考量）
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));


    // 发散函数（diverging function）绝不会返回。 它们使用 ! 标记，这是一个空类型。
    // 和所有其他类型相反，这个类型无法实例化，因为此类型可能具有的所有可能值的集合为空。 注意，它与 () 类型不同，后者只有一个可能的值。
    
    fn foo() -> ! { // 这个函数永远不会将控制内容返回给调用者
        panic!("This call never returns.");
    }
    //foo();
    // 这种类型的主要优点是它可以被转换为 "任何其他类型"，从而可以在需要精确类型的地方使用，例如在 match 匹配分支。 
    // 一个有用的例子：
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个 match 表达式的返回值必须为 u32，
            // 因为 “addition” 变量是这个类型。
            let addition: u32 = match i%2 == 1 {
                // “i” 变量的类型为 u32，这毫无问题。
                true => i,
                // 另一方面，“continue” 表达式不返回 u32，但它仍然没有问题，
                // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
