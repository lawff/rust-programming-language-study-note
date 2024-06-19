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

fn main() {
    println!("Hello, world!");

    // unwrap 方法会自动解引用到option的值
    let a = &&&&&&Some(5);

    let _b = a.unwrap();
}
