use rand::random;
use rust_programming_language_study_note::rs_15::{fibonacci, triangle, triangle_fold, I32Range};
use std::collections::{BTreeMap, HashMap};
use std::iter::{once, repeat};
use std::str::FromStr;
use std::{ffi::OsStr, iter::from_fn, path::Path};

fn main() {
    let sum = triangle(-1);
    println!("sum: {}", sum);
    let sum1 = triangle(5);
    println!("sum: {}", sum1);
    let sum2 = triangle_fold(5);
    println!("sum: {}", sum1);
    assert_eq!(sum1, sum2);

    println!("There's:");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

    for element in &v {
        println!("{}", element);
    }

    // let mut iterator = (&v).into_iter();
    // while let Some(element) = iterator.next() {
    //     println!("{}", element);
    // }

    let v = [4, 20, 12, 8, 6];
    // iter 方法返回一个不可变引用的迭代器
    // into_iter 方法会消耗集合并返回拥有所有权的迭代器
    // iter_mut 方法返回一个可变引用的迭代器
    // for 循环是调用的 into_iter 方法
    let mut iterator = v.iter();

    assert_eq!(Some(&4), iterator.next());

    let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("C:")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("JimB")));

    let s = "hello 😊"; // 注意：'😊' 是一个四字节的 UTF-8 字符

    println!("Bytes:");
    for byte in s.bytes() {
        println!("{}", byte);
    }

    println!("Characters:");
    for ch in s.chars() {
        println!("{}", ch);
    }

    let len: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(10)
        .collect();
    println!("{:?}", len);

    println!("Fibonacci: {:#?}", fibonacci().take(10).collect::<Vec<_>>());

    let mut out = "Earth".to_string();
    let inner = String::from_iter(out.drain(1..4));
    println!("out: {}, inner: {}", out, inner);

    let text = "  ponies  \n   giraffes\niguanas  \nsquid".to_string();
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    let mut x = ["earth", "water", "air", "fire"]
        .iter()
        .map(|e| println!("{}", e));
    x.next();
    x.next();
    x.next();

    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text.split_whitespace().filter_map(|w| {
        println!("{}", w);
        f64::from_str(w).ok()
    }) {
        // TODO: 为什么这里的数字输出不对？
        println!("{:4.2}", number);
    }

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }

    // 一个把城市映射为城市中停车场的表格：每个值都是一个向量
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    // 构建一个表示全部停车场的向量。`values`给出了一个能生成
    // 向量的迭代器，然后`flatten`会依次生成每个向量的元素
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();

    assert_eq!(
        all_parks,
        vec![
            "Tadasu-no-Mori Forest",
            "Maruyama Koen",
            "Percy Warner Park",
            "Dragon Park",
            "Mt. Tabor Park",
            "Forest Park"
        ]
    );

    for arg in std::env::args() {
        println!("{}", arg);
    }

    // fuse 方法会将一个迭代器转换为一个新的迭代器，这个新的迭代器会在原始迭代器返回 None 之后，一直返回 None

    let bee_parts = ["head", "thorax", "abdomen"];

    let mut iter = bee_parts.iter();
    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    let upper_case: String = "große"
        .chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after: {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROSSE");

    let v: Vec<i32> = (1..4).chain([20, 30, 40]).collect();
    assert_eq!(v, [1, 2, 3, 20, 30, 40]);

    // TODO: chunks_mut 什么意思

    let endings = ["once", "twice", "chicken soup with rice"];
    let rhyme: Vec<_> = repeat("going").zip(endings).collect();
    assert_eq!(
        rhyme,
        vec![
            ("going", "once"),
            ("going", "twice"),
            ("going", "chicken soup with rice")
        ]
    );

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });

    for line in fizz_buzz {
        println!("{}", line);
    }

    let squares = (0..10).map(|i| i * i);
    assert_eq!(squares.last(), Some(81));

    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    ["doves", "hens", "birds"]
        .iter()
        .zip(["turtle", "french", "calling"])
        .zip(2..5)
        .rev()
        .map(|((item, kind), quantity)| format!("{} {} {}", quantity, kind, item))
        .for_each(|gift| {
            println!("You have received: {}", gift);
        });

    let mut pi = 0.0;
    let mut numerator = 1.0;

    for k in (I32Range { start: 0, end: 14 }) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }
    pi *= f64::sqrt(12.0);

    println!("π ≈ {}", pi);

    // IEEE 754精确定义了此结果
    assert_eq!(pi as f32, std::f32::consts::PI);
}
