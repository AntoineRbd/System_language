struct Stack<i32> {
    stack: Vec<i32>,
}

impl<i32> Stack<i32> {
    fn new() -> Self {
        Stack {
            stack: Vec::new()
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }

    fn push(&mut self, n: i32) {
        self.stack.push(n)
    }

    fn peek(&self) -> &i32 {
        self.stack.last().unwrap()
    }
}


fn main() {
    /*
    let mut stack: Stack<isize> = Stack::new();
    stack.push(12);

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
    let mut stack: Stack<isize> = Stack::new();

    stack.push(12);
    assert_eq!(stack.stack[0], 12);

    stack.push(42);
    stack.push(2);
    assert_eq!(stack.stack[1], 42);
    assert_eq!(stack.stack[2], 2);
}

#[test]
fn stack_pop_test() {
    let mut stack: Stack<isize> = Stack::new();

    stack.push(10);
    stack.push(11);
    stack.push(12);

    assert_eq!(stack.pop(), 12);
    assert_eq!(stack.pop(), 11);
}

#[test]
fn stack_peek_test() {
    let mut stack: Stack<isize> = Stack::new();

    stack.push(12);

    assert_eq!(stack.peek(), &12);
    assert_eq!(stack.peek(), &12);
}
