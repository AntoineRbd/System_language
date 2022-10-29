fn main() {
}

#[test]
fn invert_tests() {
    assert_eq!(convert_string_to_int("0".to_string()), 0);
    assert_eq!(convert_string_to_int("-0".to_string()), 0);
    assert_eq!(convert_string_to_int("1".to_string()), 1);
    assert_eq!(convert_string_to_int("-1".to_string()), -1);
    assert_eq!(convert_string_to_int("100".to_string()), 100);
    assert_eq!(convert_string_to_int("999".to_string()), 999);
    assert_eq!(convert_string_to_int("-999".to_string()), -999);
    assert_eq!(convert_string_to_int("Antoine".to_string()), 0);
}


fn convert_string_to_int(s: String) -> i32 {
    let mut res: i32 = 0;
    let mut is_negative = false;
    let my_string: Vec<char> = s.chars().collect();

    if my_string[0] == '-' {
        is_negative = true;
    }

    for i in 0 .. s.len() {
        match my_string[i] {
        '0' => res += 0,
        '1' => res += 1,
        '2' => res += 2,
        '3' => res += 3,
        '4' => res += 4,
        '5' => res += 5,
        '6' => res += 6,
        '7' => res += 7,
        '8' => res += 8,
        '9' => res += 9,
        _ => res = res,
        }

        res *= 10;
    }

    if is_negative {
        res *= -1;
    }

    res = res / 10;

    if res == -0 {
        return res;
    }

    res
}
