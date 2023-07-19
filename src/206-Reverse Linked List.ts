class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.next = (next===undefined ? null : next)
    }
}

function reverseList(head: ListNode | null): ListNode | null {
   let previous: ListNode | null = null;
   let next: ListNode | null = null
   let node = head;
   while (node != null){
       next = node.next;
       node.next = previous;
       previous = node
       node = next;
   }
   return previous
};
