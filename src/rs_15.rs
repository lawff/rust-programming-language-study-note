pub fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    // 如果是负数的话， for 循环不会执行
    for i in 1..=n {
        sum += i;
    }
    sum
}

pub fn triangle_fold(n: i32) -> i32 {
    // (1..=n).fold(0, |sum, item| sum + item)
    (1..=n).sum()
}

pub fn triangle_sum(n: i32) -> i32 {
    (1..=n).sum()
}

// trait Iterator / IntoIterator
// trait Iterator {
//   type Item;
//   fn next(&mut self) -> Option<Self::Item>;
//   …… // 很多默认方法
// }
// next 返回 None 代表了 Iterator 已经结束了
// trait IntoIterator where Self::IntoIter: Iterator<Item=Self::Item> {
//   type Item;
//   type IntoIter: Iterator;
//   fn into_iter(self) -> Self::IntoIter;
// }

// 如果for循环中的集合是一个迭代器，那么for循环会自动调用into_iter方法，将集合转换为迭代器
// struct StudentCollection {
//   students: Vec<String>,
// }

// impl IntoIterator for StudentCollection {
//   type Item = String;
//   type IntoIter = std::vec::IntoIter<Self::Item>;

//   fn into_iter(self) -> Self::IntoIter {
//       self.students.into_iter()
//   }
// }

// fn main() {
//   let students = StudentCollection {
//       students: vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()],
//   };

//   使用 for 循环直接迭代 StudentCollection
//   for student in students {
//       println!("{}", student);
//   }
// }
pub fn dump<T, U>(t: T)
where
    T: IntoIterator<Item = U>,
    U: std::fmt::Debug,
{
    for u in t {
        println!("{:?}", u);
    }
}

pub fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    std::iter::from_fn(move || {
        let new_state = (state.1, state.0 + state.1);
        state = new_state;
        Some(state.0)
    })
}

pub struct I32Range {
    pub start: i32,
    pub end: i32,
}

impl Iterator for I32Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let result = Some(self.start);
            self.start += 1;
            result
        } else {
            None
        }
    }
}
