fn main() {
    let n = convert_int_to_string(100);
    println!("Converted int: {}", n);
}

#[test]
fn invert_tests() {
    assert_eq!(convert_int_to_string(0), "0".to_string());
    assert_eq!(convert_int_to_string(1), "1".to_string());
    assert_eq!(convert_int_to_string(-1), "-1".to_string());
    assert_eq!(convert_int_to_string(10), "10".to_string());
    assert_eq!(convert_int_to_string(100), "100".to_string());
    assert_eq!(convert_int_to_string(99999), "99999".to_string());
    assert_eq!(convert_int_to_string(-99999), "-99999".to_string());
}

fn convert_int_to_string(mut n: i32) -> String {
    let mut is_negative = false;
    let mut rest;
    let mut res = String::new();

    if n == 0 {
        res = "0".to_string();
        return res;
    }

    if n < 0 {
        n = n * (-1);
        is_negative = true;
    }

    while n != 0 {
        rest = n % 10;
        n = n / 10;
        match rest {
        0 => res.push('0'),
        1 => res.push('1'),
        2 => res.push('2'),
        3 => res.push('3'),
        4 => res.push('4'),
        5 => res.push('5'),
        6 => res.push('6'),
        7 => res.push('7'),
        8 => res.push('8'),
        9 => res.push('9'),
        _ => println!("Ain't special"),
        }
    }

    if is_negative {
        res.push('-');
    }

    res.chars().rev().collect()
}