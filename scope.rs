#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 作用域(scope)
// 它告诉编译器什么时候借用是合法的、什么时候资源可以释放、以及 变量何时被创建或销毁。
// Rust 的变量不只是在栈中保存数据：它们也占有资源，比如 Box<T> 占有 堆（heap）中的内存。
// Rust 强制实行 RAII（Resource Acquisition Is Initiallization，资源获取即初始化）
// 任何对象在离开作用域时，它的析构 函数（destructor）就被调用，然后它占有的资源就被释放。
// 避免了资源泄漏（resource leak）
// 并非所有变量都拥有资源, 比如 引用

// 所有权（ownership）
// 因为变量要负责释放它们拥有的资源（比如堆），所以资源只能拥有一个所有者。
// 在进行赋值（let x = y）或通过值来传递函数参数（foo(x)）的时候，资源 的所有权（ownership）会发生转移。按照 Rust 的说法，这被称为资源 的移动（move）。
// 注意 栈上的变量 赋值 或 传参 不会导致移动，因为栈上变量一般都实现了Copy Trait, 是拷贝赋值，不是指针赋值！
// 在移动资源之后，原来的所有者不能再被使用，这可避免悬挂指针（dangling pointer）的产生。

// 此函数取得堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
    // `c` 被销毁且内存得到释放
}

// 借用(borrow)
// 多数情况下，我们更希望能访问数据，同时不取得其所有权。
// 对象可以通过引用（&T）来传递，从而取代通过 值（T）来传递。
// 编译器（通过借用检查）静态地保证了引用总是指向有效的对象。
// 也就是说，当存在 引用指向一个对象时，该对象不能被销毁。 (静态 引用计数 技术)
// &T 通过不可变引用（immutable reference）来借用数据，借用者可以读数据而不能更改数据
fn borrow_i32(borrowed_i32: &i32) {
    // 请注意函数自身就是一个作用域，运行完成以后，在函数中临时创建的引用也就不复存在了。
    println!("This int is: {}", borrowed_i32);
}

// 可变性 (mutability)
// 可变数据可以使用 &mut T 进行可变借用。这叫做可变引用(mutable reference)
fn mut_borrow_i32(mut_i32: &mut i32) {
    *mut_i32 += 1; // 注意使用 解引用 取得其值
}

// 析构函数 (drop/destruct function)
// 通过 Drop trait 提供的. 当资源离开作用域，就调用析构函数。
// 无需为每种类型都实现 Drop trait，只要为那些需要 自己的析构函数逻辑 的类型实现就可以了。
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = 5u32; // 栈分配的整型
    let y = x; // 将 `x` *复制*到 `y`——不存在资源移动
    println!("x is {}, and y is {}", x, y); // 两个值各自都可以使用

    let a = Box::new(5i32); // `a` 是一个指向堆分配的整数的指针
    // move occurs because `a` has type `Box<i32>`, which does not implement the `Copy` trait
    let b = a; // *移动* `a` 到 `b`
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向同一个堆分配的数据，但是现在是 `b` 拥有它。

    // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存
    //println!("a contains: {}", a); // error: value borrowed here after move!

    // 此函数从 `b` 中取得堆分配的内存的所有权
    destroy_box(b);

    // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
    //println!("b contains: {}", b); // 报错！和前面出错的原因一样。

    // 当所有权转移时，数据的可变性可能发生改变。
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);
    // 可变性错误
    //*immutable_box = 4; // error: cannot assign, as `immutable_box` is not declared as mutable

    // *移动* box，改变所有权（和可变性）
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);
    // 可以修改 box 的内容
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;
    borrow_i32(&boxed_i32); // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用。
    borrow_i32(&stacked_i32); // 传引用到引用参数
    //borrow_i32(stacked_i32); // error: expected `&i32`, found `i32`

    {
        let _ref_to_i32: &i32 = &boxed_i32; // borrow of `boxed_i32` occurs here, 取得一个对 box 中数据的引用
        // 报错！当 `boxed_i32` 里面的值之后在作用域中被借用时，不能将其销毁。
        //destroy_box(boxed_i32); // error: move out of `boxed_i32` occurs here
        borrow_i32(_ref_to_i32); // borrow later used here
        // `_ref_to_i32` 离开作用域且不再被借用。
    }

    destroy_box(boxed_i32); // 能够销毁, 是因为后面已经不存在对 `boxed_i32` 的引用

    let mut mut_stacked_i32 = stacked_i32;
    // 不可变地借用一个不可变对象
    borrow_i32(&stacked_i32);
    // 不可变地借用一个可变对象
    borrow_i32(&mut_stacked_i32);
    // 报错！不能可变地借用一个不可变对象
    //mut_borrow_i32(&mut stacked_i32); // error: cannot borrow as mutable
    // 可变地借用一个可变对象
    mut_borrow_i32(&mut mut_stacked_i32);

    // 冻结(freeze)
    // 当数据被不可变地借用时，它还会冻结（freeze）。
    // 已冻结的数据无法通过 原始对象 来 修改 ，直到对这些数据的所有引用离开作用域为止。
    // 1.在不可变借用的期间，原始数据不可进行可变借用。
    // 2.只有在可变引用离开作用域之后，原始数据才可再次被借用 或 可变借用。
    // 在同一时刻内只允许有 一个 可变借用。并且可变借用和不可变借用不能同时存在。
    let mut _mutable_int = 7i32;
    {
        let large_integer = &_mutable_int; // immutable borrow of `_mutable_int` occurs here
        
        // 报错！`_mutable_integer` 在本作用域被冻结
        // cannot assign to `_mutable_int` because it is borrowed
        //_mutable_int = 50; // error: assignment to borrowed `_mutable_int` occurs here

        // 报错！不能可变地借用 `_mutable_int` ，因为现在它有不可变的借用。
        //let another_mutable_int = &mut _mutable_int; // error: mutable borrow occurs here

        println!("Immutably borrowed {}", large_integer); // immutable borrow later used here
        // `large_integer` 离开作用域
    }
    // 正常运行！`_mutable_integer` 在这作用域没有冻结
    _mutable_int = 3;

    // ref也可以用来创建引用，通常用于结构体 解构
    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    println!("tuple is {:?}", mutable_tuple);
    {
        // 解构 `mutable_tuple` 来改变 `last` 的值。
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);

    // 构造变量
    let x = ToDrop;
    println!("Made a ToDrop!");
    // x在此处析构
}
