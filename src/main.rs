// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {
    println!("Hello, world!");
    println!("{}",reverse_list([0,1,2,4,10,4,30].to_vec()));
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut node = head;
    let mut prev = None;
    while let Some(mut boxed_node) = node {
        let next = boxed_node.next.take();
        boxed_node.next = prev;
        prev = Some(boxed_node);
        node = next;
    }
    prev
}