#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// rust 模块: 是item的集合，包括函数、结构体、trait、impl块 或 其他模块
// 可见性: 默认：模块中的item私有课件，加上pub之后可以公有可见。

// 一个文件可以是一个模块，此时不需要模块声明。
// 如果要在文件中引用另一文件的方法，使用 mod my; 进行声明。
// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容放到 本文件作用域中一个名为 `my` 的模块里面。

// 结构体可见性: 和C++的类相似
// 字段默认拥有私有的可见性，也可以加上 pub 修 饰语来重载该行为。
// 只有从结构体被定义的模块之外访问其字段时，这个可见性才会 起作用，其意义是隐藏信息（即封装，encapsulation）。

// crate 编译链接库:
// 创建库文件:
// > rustc --crate-type=lib rary.rs
// 或产生 library.rlib 文件, 增加了前缀lib!
// 其他文件中使用 extern crate rary; 链接到 `rary` 库，导入其中的项
// 并使用 rary:: 来访问该库中项的成员
// 编译选项:
// > rustc executable.rs --extern rary=library.rlib

mod my_mod {
    fn private_func() {
        println!("called `my_mod::private_function()`");
    }
    pub fn public_func() {
        println!("called `my_mod::function()`");
    }
    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_func();
    }
    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
            super::private_func(); // super可以访问父类方法，包括私有
        }
        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            self::public_function_in_nested() // `self` 关键字表示当前的模块作用域
        }
        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }
        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }
    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }
    // `pub(crate)` 使得函数只在当前 crate 中可见.
    // crate（中文有 “包，包装箱” 之意）是 Rust 的编译单元。
    // 当调用 rustc some_file.rs 时，some_file.rs 被当作 crate 文件。
    // 如果 some_file.rs 里面含有 mod 声明，那么模块文件的内容将在编译之前被插入 crate 文件的相应声明处。
    // 换句话说，模块不会单独被编译，只有 crate 才会被编译。
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }
    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

mod my {
    // 一个公有的结构体，带有一个公有的字段
    pub struct OpenBox<T> {
        pub contents: T,
    }
    // 一个公有的结构体，带有一个私有的字段
    pub struct ClosedBox<T> {
        contents: T,
    }
    impl<T> ClosedBox<T> {
        // 一个公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }

}

fn main() {
    // 将 `my_mod::nested::function` 路径绑定到 `other_function`。
    // `use` 绑定拥有局部作用域。在这个例子中，`function()` 的掩蔽只存在在这个代码块中。
    use my_mod::nested::function as other_function;
    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    other_function();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    //my_mod::nested::public_function_in_my_mod(); // error: private function
    //my_mod::private_nested::function(); // error: private module

    // 带有公有字段的公有结构体，可以像平常一样构造
    let open_box = my::OpenBox{ contents: "public infomation" };
    println!("The open box contains: {}", open_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造。
    //let closed_box = my::ClosedBox { contents: "classified information" }; // error: field `contents` of struct `ClosedBox` is private
    let _closed_box = my::ClosedBox::new("classified information");
    // 并且一个结构体中的私有字段不能访问到。(访问: 读写)
    //println!("The closed box contains: {}", _closed_box.contents); // error: field `contents` of struct `ClosedBox` is private
}
