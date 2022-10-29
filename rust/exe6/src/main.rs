const LENGTH: usize = 10;

struct Stack {
    stack: [i32; LENGTH],
    index: usize,
}

impl Stack {
  fn new() -> Self {
    Stack {
        stack: [-1; LENGTH],
        index: 0,
    }
  }

  fn pop(self: &mut Stack) -> i32 {
      if self.index > 0 {
          let res = self.stack[self.index - 1];
          self.index -= 1;
          return res
      }
      else {
          return -1;
      }
  }

  fn push(self: &mut Stack, n: i32) {
      if self.index < 10 {
          self.stack[self.index] = n;
          self.index += 1;
      }
  }

  fn peek(self: &mut Stack) -> i32 {
      if self.index > 0 {
          return self.stack[self.index - 1];
      }
      else {
          return -1;
      }
  }
}


fn main() {
    /*
    let mut stack: Stack = Stack::new();

    stack.push(12);
    stack.push(42);
    stack.push(50);

    stack.print();

    let i = stack.pop();
    println!("Item remove = {}", i);
    stack.print();
    */
}

#[test]
fn stack_push_test() {
    let mut stack: Stack = Stack::new();

    stack.push(12);
    assert_eq!(stack.stack[0], 12);

    stack.push(42);
    stack.push(2);
    assert_eq!(stack.stack[1], 42);
    assert_eq!(stack.stack[2], 2);
}

#[test]
fn stack_push_limit_test() {
    let mut stack: Stack = Stack::new();

    for _i in 0..10 {
        stack.push(10);
    }

    stack.push(42);
    assert_eq!(stack.stack[9], 10);
}

#[test]
fn stack_pop_test() {
    let mut stack: Stack = Stack::new();

    stack.push(10);
    stack.push(11);
    stack.push(12);

    assert_eq!(stack.pop(), 12);
    assert_eq!(stack.pop(), 11);
}

#[test]
fn stack_pop_limit_test() {
    let mut stack: Stack = Stack::new();

    stack.push(10);

    assert_eq!(stack.pop(), 10);
    assert_eq!(stack.pop(), -1);
}

#[test]
fn stack_peek_test() {
    let mut stack: Stack = Stack::new();

    stack.push(12);

    assert_eq!(stack.peek(), 12);
    assert_eq!(stack.peek(), 12);
}

#[test]
fn stack_peek_limit_test() {
    let mut stack: Stack = Stack::new();

    assert_eq!(stack.peek(), -1);
}
