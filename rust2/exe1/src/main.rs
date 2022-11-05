pub struct SortedList {
    list: Vec<i32>,
}

impl SortedList {
    fn new() -> Self {
        SortedList {
            list: Vec::new(),
        }
    }

    fn insert(&mut self, value: i32) {
        self.list.push(value);
        self.list.sort();
    }

    /*
    fn print(&self) {
        println!("list: {:?}", self.list);
    }
    */
}


fn main() {
}

#[test]
fn test_insertion() {
    let mut list: SortedList = SortedList::new();

    list.insert(1);
    list.insert(0);
    list.insert(12);
    list.insert(42);
    list.insert(-12);
    list.insert(7);

    assert_eq!(list.list, vec![-12, 0, 1, 7, 12, 42]);
}
