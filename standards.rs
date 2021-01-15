#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
// 标准库

// Box<T>: 智能指针，指向堆分配 的 T 类型的值. 
// 当箱子离开作用域时，它的析构函数会被调用，内部的对象会被 销毁，堆上分配的内存也会被释放。
// box 的宽度就是指针宽度
// 使用解引用 * 获得其值

// vector: 大小可变的数组, 数据分配在堆上
// 使用 3 个词来表示：一个指向数据的指针，vector 的长度，还有它的容量。

// &[T] 是 Vec<T> 的切片

// String: 存储为由字节组成的 vector（Vec<u8>）
// 堆分配的，可增长的，且不是零结尾的

// &str 是一个总是指向有效 UTF-8 序列的切片(&[u8])
// 并可用来查看 String 的内容

// HashMap 用散列表实现
// HashMap 的键可以是布尔型、整型、字符串，或任意实现了 Eq 和 Hash trait 的其他类型
// 可以使用 HashMap::with_capacity(unit) 创建具有一定初始容量的 HashMap，
// 也可以使用 HashMap::new() 来获得一个带有默认初始容量的 HashMap
// 注意到 f32 和 f64 没有实现 Hash，这很大程度上是由于若使用浮点数作为 散列表的键，浮点精度误差会很容易导致错误。
// 对自定义类型可以轻松地实现 Eq 和 Hash，只需加上一行代码：#[derive(PartialEq, Eq, Hash)]。

// HashSet 集合
// HashSet<T> 实际上只是对 HashMap<T, ()> 的封装
// 保证了不会出现重复的元素
//  4 种基本操作
// 1. union（并集）：获得两个集合中的所有元素（不含重复值）。
// 2. difference（差集）：获取属于第一个集合而不属于第二集合的所有元素。
// 3. intersection（交集）：获取同时属于两个集合的所有元素。
// 4. symmetric_difference（对称差）：获取所有只属于其中一个集合，而不同时属于 两个集合的所有元素。
// 上述操作都不会改变 原集合 的值

fn main() {
    // 迭代器可以被收集到 vector 中
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // `vec!` 宏可用来初始化一个 vector
    let mut xs = vec![1i32, 2, 3];
    xs.push(4);
    println!("Vector: {:?}", xs);
    println!("Vector size: {}", xs.len());
    println!("Second element: {}", xs[1]);
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }
    // 多亏了 `iter_mut`，可变的 `Vector` 在迭代的同时，其中每个值都能被修改
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Pop last element: {:?}", xs.pop());
    // 堆上数组越界触发 运行时错误，编译器不做检查
    //println!("Fourth element: {}", xs[3]); // panic: index out of bounds: the len is 3 but the index is 3

    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    // 逆序迭代单词，这里并没有分配新的字符串
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }
    
    // 复制字符到一个 vector，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // 创建一个空的且可增长的 `String`
    let mut string = String::new();
    for c in chars {
        string.push(c); // 在字符串的尾部插入一个字符
        string.push_str(", "); // 在字符串尾部插入一个字符串
    }
    
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim); // 依然返回一个引用(切片), 而不是重新分配存储
    println!("Used characters: {}", trimmed_str);

    // 堆分配一个字符串
    // 此时alice和bob各自拥有对应的资源
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串，不是在原来字符串上修改
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    // HashMap
    use std::collections::HashMap;
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");
    // 接受一个引用并返回 Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", number),
        _ => println!("Don't have Daniel's number."),
    }
    // 如果被插入的值为新内容，那么 `HashMap::insert()` 返回 `None`，
    // 否则返回 `Some(value)`
    contacts.insert("Daniel", "164-6743");
    contacts.remove(&("Ashley")); // 传引用, 效率更高
    contacts.remove("Ashley"); // 传值
    // `HashMap::iter()` 返回一个迭代器，该迭代器以 任意顺序 (不是按插入顺序！) 举出 (&'a key, &'a value) 对。
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, number); 
    }

    // HashSet
    use std::collections::HashSet;
    let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    let b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();
    // 如果值已经存在，那么 `HashSet::insert()` 返回 false。
    assert!(a.insert(4));
    assert!(a.contains(&4));
    println!("A: {:?}", a);
    // 乱序打印 [1, 2, 3, 4]。
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    // 这将会打印出 [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!("A: {:?}", a);
    // 乱序打印 [2, 3, 4]。
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());
    // 打印 [1]
    println!("Symmetric Difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
