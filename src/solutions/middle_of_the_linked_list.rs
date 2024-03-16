//  876. Middle of the Linked List
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
pub fn middle_node(
    head: Option<Box<Solution::ListNode>>,
) -> Solution::Option<Box<Solution::ListNode>> {
    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    return slow.clone();
}
