fn main() {
  println!("Hello, world!");
  let mut obj = MinStack::new();
  obj.push(0);
  obj.push(1);
  obj.push(0);
  obj.get_min();
  obj.pop();
  obj.get_min();
}

struct MinStack {
  stack: Vec<i32>,
  minStack: Vec<i32>
}


/** 
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl MinStack {

  fn new() -> Self {
      MinStack 
      { 
          minStack: Vec::new(),
          stack: Vec::new()
      }
  }
  
  fn push(&mut self, val: i32) {
      if self.minStack.len() == 0 {
          self.minStack.push(val)
      }
      else if self.minStack[self.minStack.len() -1]  > val {
          self.minStack.push(val)
      } else {
          self.minStack.push(self.minStack[self.minStack.len() -1])
      }
      self.stack.push(val);
  }
  
  fn pop(&mut self) {
      let pop = self.stack.pop();
      if pop == Some(self.minStack[self.minStack.len() -1]) {
          self.minStack.pop();
      }
  }
  
  fn top(&self) -> i32 {
      self.stack[self.stack.len() - 1]
  }
  
  fn get_min(&self) -> i32 {
      self.minStack[self.minStack.len() -1]
  }
}


