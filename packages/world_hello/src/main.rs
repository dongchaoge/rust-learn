#![allow(unused_variables)]
// fn greet_world() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界，你好";
//     let english = "World, hello";
//     let regions = [southern_germany, chinese, english];
//     for region in regions {
//         println!("{}", &region);
//     }
// }

// fn main() {
//     greet_world();
// }

// -----

// fn main() {
//     let penguin_data = "\
//     common name,length (cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";

//     let records = penguin_data.lines();

//     eprintln!("list-----{:?}/n---", records);

//     for (i, record) in records.enumerate() {
//         if i == 0 || record.trim().len() == 0 {
//             continue;
//         }

//         // 声明一个 fields 变量，类型是 Vec
//         // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//         // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
//         let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
//         if cfg!(debug_assertions) {
//             // 输出到标准错误输出
//             eprintln!("debug: {:?} -> {:?}", record, fields);
//         }

//         let name = fields[0];
//         // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
//         //
//         // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
//         //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
//         //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
//         //
//         // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
//         if let Ok(length) = fields[1].parse::<f32>() {
//             // 输出到标准输出
//             println!("{}, {}cm", name, length);
//         }
//     }
// }

// -----

// // Rust 程序入口函数，跟其它语言一样，都是 main，该函数目前无返回值
// fn main() {
//     // 使用let来声明变量，进行绑定，a是不可变的
//     // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
//     // 语句的末尾必须以分号结尾
//     let a = 10;
//     // 主动指定b的类型为i32
//     let b: i32 = 20;
//     // 这里有两点值得注意：
//     // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
//     // 2. c是可变的，mut是mutable的缩写
//     let mut c = 30i32;
//     // 还能在数值和类型中间添加一个下划线，让可读性更好
//     let d = 30_i32;
//     // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
//     c = 40

//     let e = add(add(a, b), add(c, d));

//     // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
//     // 该函数将指定的格式化字符串输出到标准输出中(控制台)
//     // {}是占位符，在具体执行过程中，会把e的值代入进来

//     println!("( a + b ) + ( c + d ) = {}", e);
// }

// // 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
// fn add(i: i32, j: i32) -> i32 {
//     // 返回相加值，这里可以省略return
//     i + j
// }

// -----

// struct Struct {
//     e: i32,
// }
// use num::complex::Complex;

// #[derive(Debug, Clone)]
// struct File {
//     name: String,
//     data: Vec<u8>,
// }

// #[allow(dead_code)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
// let (a, b, c, d, e);

// (a, b) = (1, 2);
// // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
// [c, .., d, _] = [1, 2, 3, 4, 5, 6];
// Struct { e, .. } = Struct { e: 5 };

// assert_eq!([1, 2, 1, 5, 5], [a, b, c, d, e]);

// let a: u8 = 255;
// let b = a.overflowing_add(20);
// println!("{:?}", b); // 19

// // 对于较长的数字，可以用_进行分割，提升可读性
// let one_million: i64 = 1_000_000;
// println!("{}", one_million.pow(2));

// // 定义一个f32数组，其中42.0会自动被推导为f32类型
// let forty_twos: [f32; 3] = [42.0, 42f32, 42.0_f32];

// // 打印数组中第一个值，并控制小数位为2位
// println!("{:.2}", forty_twos[0]);

// for i in 1..=5 {
//     println!("{}", i)
// }

// let a = Complex { re: 2.1, im: -1.2 };
// let b = Complex::new(11.1, 22.2);
// let result = a + b;

// println!("{} + {}i", result.re, result.im);

// let number = 'a' as u8;
// println!("{}", number);

// let mut s = String::from("hello");

// let len = s.len();

// let slice2 = &s[..];

// println!("{}", slice2);

// s.push_str(" ssg");

// println!("{}", &s[..]);

// let mut string_pop = String::from("rust pop 中文!");
// let p1 = string_pop.pop();
// let p2 = string_pop.pop();
// dbg!(p1);
// dbg!(p2);
// dbg!(string_pop);

// let f1 = File {
//     name: String::from("f1.txt"),
//     data: Vec::new(),
// };

// let f1_name = f1.clone().name;
// let f1_length = &f1.data.len();

// println!("{:?}", f1);
// println!("{} is {} bytes long", f1_name, f1_length);

// let msg = Message::Move { x: 1, y: 1 };

// if let Message::Move { x: a, y: b } = msg {
//     assert_eq!(a, b);
// } else {
//     panic!("不要让这行代码运行！");
// }

// for i in 1..=5 {
//     println!("{}", i);
// }

// let a = [4, 3, 2, 1];
// // `.iter()` 方法把 `a` 数组变成一个迭代器
// for (i, v) in a.iter().enumerate() {
//     println!("第{}个元素是{}", i + 1, v);
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn new(width: u32, height: u32) -> Self {
//         Rectangle { width, height }
//     }
//     fn get_width(&self, num: u32) -> u32 {
//         self.width + num
//     }
//     fn get_self_num(num: u32) -> u32 {
//         num
//     }
// }

// let rect1 = Rectangle::new(30, 50);

// println!("{}", rect1.get_width(10));
// println!("{}", rect1.height);
// println!("{}", Rectangle::get_self_num(2));

// use std::collections::HashMap;

// let text = "hello world wonderful world";

// let mut map = HashMap::new();
// // 根据空格来切分字符串(英文单词都是通过空格切分)
// for word in text.split_whitespace() {
//     let count = map.entry(word).or_insert(0);
//     *count += 1;
// }

// println!("{:?}", map);

// let string1 = String::from("long string is long");
// let result;
// {
//     let string2 = String::from("xyz");
//     result = longest(string1.as_str(), string2.as_str());
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// }

// #![allow(dead_code)]

// use std::fmt;
// use std::fmt::Display;

// #[derive(Debug, PartialEq)]
// enum FileState {
//     Open,
//     Closed,
// }

// #[derive(Debug)]
// struct File {
//     name: String,
//     data: Vec<u8>,
//     state: FileState,
// }

// impl Display for FileState {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             FileState::Open => write!(f, "OPEN"),
//             FileState::Closed => write!(f, "CLOSED"),
//         }
//     }
// }

// impl Display for File {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "<{} ({})>", self.name, self.state)
//     }
// }

// impl File {
//     fn new(name: &str) -> File {
//         File {
//             name: String::from(name),
//             data: Vec::new(),
//             state: FileState::Closed,
//         }
//     }
// }

// fn main() {
//     let f6 = File::new("f6.txt");
//     //...
//     // 这是原生的
//     println!("{:?}", f6);
//     // 这是自定义的
//     println!("{}", f6);
// }

// // use std::collections::HashMap;
// fn main() {
//     let names = ["sunface", "sunfei"];
//     let ages = [18, 18];
//     // let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
//     let folks: Vec<_> = names.into_iter().zip(ages.into_iter()).collect();

//     println!("{:?}", folks[0].0);
// }
