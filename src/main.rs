// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn training_plan(head: Option<Box<ListNode>>, cnt: i32) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;

        for _i in 0..cnt {
            match fast {
                Some(node) => {
                    fast = &node.next;
                }
                None => return None,
            }
        }

        // 让fast和slow同时移动，直到fast到达链表末尾
        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }

        slow.clone()
    }
}

// fn main() {
//     let stdin = std::io::stdin();
//     println!("{}", stdin.lock().lines().count());
// }

fn main() {
    let id = "Iterator";
    let mut chars = id.chars();

    // 使用 `any` 检查是否有大写字母
    assert!(chars.by_ref().any(char::is_uppercase));

    // 现在再次使用迭代器
    let remaining_chars: String = chars.collect();
    println!("Remaining characters: {}", remaining_chars);
}
