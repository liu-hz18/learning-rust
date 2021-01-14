// 条件编译可能通过两种不同的操作符实现
// cfg 属性：在属性位置中使用 #[cfg(...)]
// cfg! 宏：在布尔表达式中使用 cfg!(...)
// 也可以自定义编译选项，使用--cfg some_config 在编译时设定
#[cfg(test)]
mod tests { // 单元测试写在本模块中，集成测试放在/tests文件夹下
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// 这个函数仅当目标系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// 而这个函数仅当目标系统 **不是** Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

// 新项目的入口源文件
fn main() {
    println!("Hello, world!");
    are_you_on_linux();
    // cfg! 宏：在布尔表达式中使用 cfg!(...)
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
