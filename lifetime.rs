#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 生命周期(lifetime) （它们本身就是泛型）
// 借用检查器 使用显式的生命周期标记来明确引用的有效时间应该持续多久。
// 可以用 撇号 显式地标出生命周期
// foo<'a>
// `foo` 带有一个生命周期参数 `'a`, 也表明了 foo 的生命周期不能超出 'a 的周期。
// foo<'a, 'b>
// `foo` 带有生命周期参数 `'a` 和 `'b`
// 在上面这种情形中，foo 的生命周期不能超出 'a 和 'b 中任一个的周期。
// lifetime 约束: `<'a: 'b, 'b>` 读作生命周期 `'a` 至少和 `'b` 一样长。
// 一个较长的生命周期可以强制转成一个较短的生命周期
// 反过来不可以

// 带上生命周期的函数签名有一些限制
// 1. 任何引用都必须拥有标注好的生命周期。
// 2. 任何被返回的引用都必须有和某个输入量相同的生命周期或是静态类型（static）。

// `print_refs` 接受两个 `i32` 的引用，它们有不同的生命周期 `'a` 和 `'b`。
// 这两个生命周期都必须至少要和 `print_refs` 函数一样长。
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// 不带参数的函数，不过有一个生命周期参数 `'a`。
fn failed_borrow<'a>() { // lifetime `'a` defined here
    let _x = 12;

    // 报错：`_x` 的生命周期不够长
    //let y: &'a i32 = &_x; // error: borrowed value does not live long enough
    // 在函数内部使用生命周期 `'a` 作为显式类型标注将导致失败，因为 `&_x` 的
    // 生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期。
}

// 在 结构体 中标注生命周期也和函数的类似
// 一个 `Borrowed` 类型，含有一个指向 `i32` 类型的引用。
// 该引用 &i32 必须比 `Borrowed` 寿命更长。
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Trait/impl/method 都可以有声明周期标注
// 给 impl 标注生命周期。
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self(&10)
    }
}

// 'static 生命周期是可能的生命周期中最长的，它会在整个程序运行的时期中存在。
// 'static 把数据保存在可执行文件的"只读内存区"
// 1. 使用 static 声明来产生常量（constant）。
// 2. 产生一个拥有 &'static str 类型的 string 字面量。

// 产生一个拥有 `'static` 生命周期的常量。
static NUM: i32 = 18;
// 返回一个指向 `NUM` 的引用，该引用不取 `NUM` 的 `'static` 生命周期，
// 而是被强制转换成和输入参数的一样。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}


fn main() {
    // 创建变量，稍后用于借用。
    let (four, nine) = (4, 9);
    // 两个变量的借用（`&`）都传进函数。
    print_refs(&four, &nine);
    // 任何被借用的输入量都必须比借用者生存得更长。
    // 也就是说，`four` 和 `nine` 的生命周期都必须比 `print_refs` 的长。

    failed_borrow();
    // `failed_borrow` 未包含引用，因此不要求 `'a` 长于函数的生命周期，
    // 但 `'a` 寿命确实更长。因为该生命周期从未被约束，所以默认为 `'static`。

    let x = 18;
    let single = Borrowed(&x);
    println!("x is borrowed in {:?}", single);

    let b: Borrowed = Default::default();
    println!("b is {:?}", b);

    {
        // 产生一个 `string` 字面量并打印它：static_string 类型是 &'static str
        let static_string = "I'm in read-only memory"; // 字符串字面量(string literals), 来自只读存储（preallocated text），static_string是它的切片(引用)
        println!("static_string: {}", static_string);

        // 当 `static_string` 离开作用域时，该引用不能再使用，不过
        // 数据仍然存在于二进制文件里面。
    }
    println!("NUM: {} stays accessible!", NUM);
}
