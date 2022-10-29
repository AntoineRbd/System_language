struct Stack {
    stack: [i32; 0],
}

impl Stack {
  fn new() -> Self {
    Stack {stack: []}
  }
/*
  fn length(&self) -> usize {
    self.stack.len()
  }
  */

  fn pop(&mut self) -> i32 {
    let (self.stack, end)  = pop_right(self.stack);
    end
  }

  fn push(&mut self, n: i32) {
    self.stack = push_right(self.stack, n);
  }
  /*

  fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }

  fn peek(&self) -> Option<&T> {
    self.stack.last()
  }
  */
}


fn main() {
    let mut vals: Stack = Stack::new();
    //let vals: [i32; 0] = [];
    //let stack = Stack { stack: vals};

    println!("OK!");
}
