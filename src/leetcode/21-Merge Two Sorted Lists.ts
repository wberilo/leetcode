
class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
      this.val = (val===undefined ? 0 : val)
      this.next = (next===undefined ? null : next)
  }
}


function mergeTwoLists(list1: ListNode | null, list2: ListNode | null): ListNode | null {

let node: null |ListNode = null;


let init: null |ListNode = null;


while (list1 !== null && list2 !== null) {
  if (node === null) {
    if (list1.val <= list2.val) {
      init = list1;
      node = list1;
      list1 = list1.next;
    }
    else {
      init = list2;
      node = list2;
      list2 = list2.next;
    }
  }

  else {
    if (list1.val <= list2.val) {
      node.next = list1;
      list1 = list1.next;
    } else {
      node.next = list2;
      list2 = list2.next;
    }
    node = node.next
  }
}

return init;
};