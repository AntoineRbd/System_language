fn main() {
}


#[test]
fn invert_tests() {
    assert_eq!(invert(&"kayak".to_string()), "kayak".to_string());
    assert_eq!(invert(&"hello world".to_string()), "dlrow olleh".to_string());
}
#[test]
fn invert_place_tests() {
    let mut s = String::from("kayak");
    invert_place(&mut s);

    assert_eq!(s, "kayak".to_string());
    
    s = String::from("hello world");
    invert_place(&mut s);
    assert_eq!(s, "dlrow olleh".to_string());
}


fn invert(s: &String) -> String {
    let mut res = String::from("");
    let my_string: Vec<char> = s.chars().collect();

    for i in (0..s.len()).rev() {
        res.push(my_string[i]);
    }

    res
}

fn invert_place(s: &mut String) {
    let tmp = invert(s);
    *s = tmp;
}