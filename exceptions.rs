#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// 最简单的错误处理机制就是 panic
// 它会打印一个错误消息，开始 回退（unwind）任务，且通常会退出程序。
// 在回退栈 的同时，运行时将会释放该线程所拥有的所有资源，这是通过调用线程中所有对象的 析构函数完成的。
// panic! 不会泄露内存
fn give_princess_1(gift: &str) {
    if gift == "snake" {
        panic!("AAAAAAAAA!!!!");
    }
    println!("I love {} !!!!", gift);
}

// Option<T> 枚举 类型
// 用于有 “不存在” 的可能性的情况。
// 1. Some(T)：找到一个属于 T 类型的元素
// 2. None：找不到相应元素
// 这些选项可以通过 match 显式地处理，或使用 unwrap 隐式地处理。
// 隐式处理要么 返回 Some 内部的元素，要么就 panic。

// 所有礼物都显式地使用 `match` 来处理。
fn give_commoner(gift: Option<&str>) {
    // 指出每种情况下的做法。
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner)   => println!("{}? How nice.", inner), // 表示 enum的解构
        None          => println!("No gift? Oh well."),
    }
}

// 这里所有的礼物都使用 `unwrap` 隐式地处理。
fn give_princess_2(gift: Option<&str>) {
    // `unwrap` 在接收到 `None` 时将返回 `panic`。
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }
    println!("I love {}!!!!!", inside);
}

// Result (枚举类型) 是 Option 类型的更丰富的版本，描述的是可能的错误而不是可能的不存在。
// Result<T，E> 可以有两个结果的其中一个
// 1. Ok<T>：找到 T 元素， 表示操作成功，并包装操作返回的 value
// 2. Err<E>：找到 E 元素，E 即表示错误的类型。表示操作失败，并包装 why，它（但愿）能够解释失败的原因。（why 拥有 E 类型）
// 例如 parse() 方法。它并不是总能把字符串解析成指定的类型，所以 parse() 返回一个 Result 表示可能的失败。
fn multiply_1(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

// 当找到一个 Err 时，可以采取两种行动：
// 1. panic!，不过我们已经决定要尽可能避免 panic 了。
// 2. 返回它，因为 Err 就意味着它已经不能被处理了。
// ? 几乎就等于一个会返回 Err 而不是 panic 的 unwrap。
// ? 要么 unwrap 要么 return Err(error)
// 它等同于这样一个匹配 表达式：其中 Err(err) 分支展开成提前返回的 return Err(err)，而 Ok(ok) 分支展开成 ok 表达式。

// 别名
use std::num::ParseIntError;
// 为带有错误类型 `ParseIntError` 的 `Result` 定义一个泛型别名。
type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply_2(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    give_princess_1("teddy bear");
    //give_princess_1("snake"); // thread 'main' panicked at 'AAAAAAAAA!!!!'

    let food  = Some("chicken");
    let snake = Some("snake");
    let void  = None;
    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing: Option<&str> = None;
    let another_nothing = None::< Option<&str> >; // 另一种表示

    give_princess_2(bird);
    //give_princess_2(nothing); // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'

    let twenty = multiply_1("10", "2");
    println!("double is {}", twenty);

    //let tt = multiply("t", "2"); // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }'
    //println!("double is {}", tt);
    print(multiply_2("10", "2"));
    //print(multiply_2("t", "2")); // Error: invalid digit found in string
}
