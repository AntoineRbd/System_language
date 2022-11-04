pub struct SortedList<K: Ord, V: PartialEq> {
    list: Vec<(K, V)>,
    index: usize,
}

impl<K: Ord, V: PartialEq> SortedList<K, V> {
    fn new() -> Self {
        SortedList {
            list: Vec::new(),
            index: 0,
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.list.push((key, value));
        //self.list.sort_by_key(|k| k.abs());
    }

    fn iter(&mut self) -> (K, V) {
        let i = self.list[self.index];
        self.index += 1;
        return i;
    }


    fn print(&self) {
        //println!("List: {:#?}", self.list);
    }
}


fn main() {
    let mut list: SortedList<i32, i32> = SortedList::new();
    list.insert(1, 1);
    list.insert(0, 3);
    list.insert(2, 12);

    assert_eq!(list.iter().collect::<Vec<_>>(), vec![(&1, 1), (&0, 3), (&2, 12)]);
}
