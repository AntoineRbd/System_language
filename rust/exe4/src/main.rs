use std::result;

fn main() {
    //let n = "II".to_string();
    //let res = convert_roman_numerals_to_int(&n);

    /*
    let n: i32 = 0;
    let res = convert_int_to_roman_numerals(n);
    match res {
        Ok(v) => println!("Int converted {} to {v:?}", n),
        Err(e) => println!("Int converted {} to {e:?}", n),
    }
    */

}

#[test]
fn conversion_tests() {
    /* Unit */
    assert_eq!(convert_int_to_roman_numerals(0), Err("Invalid number: number 0 does not exist in roman numerals"));
    assert_eq!(convert_int_to_roman_numerals(1), Ok("I".to_string()));
    assert_eq!(convert_int_to_roman_numerals(2), Ok("II".to_string()));
    assert_eq!(convert_int_to_roman_numerals(3), Ok("III".to_string()));
    assert_eq!(convert_int_to_roman_numerals(4), Ok("IV".to_string()));
    assert_eq!(convert_int_to_roman_numerals(5), Ok("V".to_string()));
    assert_eq!(convert_int_to_roman_numerals(6), Ok("VI".to_string()));
    assert_eq!(convert_int_to_roman_numerals(7), Ok("VII".to_string()));
    assert_eq!(convert_int_to_roman_numerals(8), Ok("VIII".to_string()));
    assert_eq!(convert_int_to_roman_numerals(9), Ok("IX".to_string()));
    assert_eq!(convert_int_to_roman_numerals(10), Ok("X".to_string()));

    /* Random number */
    assert_eq!(convert_int_to_roman_numerals(39), Ok("XXXIX".to_string()));
    assert_eq!(convert_int_to_roman_numerals(246), Ok("CCXLVI".to_string()));
    assert_eq!(convert_int_to_roman_numerals(789), Ok("DCCLXXXIX".to_string()));

    /* Number with 0 */
    assert_eq!(convert_int_to_roman_numerals(160), Ok("CLX".to_string()));
    assert_eq!(convert_int_to_roman_numerals(207), Ok("CCVII".to_string()));

    /* Thousands numbers */
    assert_eq!(convert_int_to_roman_numerals(1918), Ok("MCMXVIII".to_string()));
    assert_eq!(convert_int_to_roman_numerals(2014), Ok("MMXIV".to_string()));

    /* Negative numbers */
    assert_eq!(convert_int_to_roman_numerals(-12), Err("Invalid number: negative number does not exist in roman numerals"));
}

#[test]
fn convert_roman_numerals_to_int_test() {
    /* Unit */
    assert_eq!(convert_roman_numerals_to_int(&"0".to_string()), Err("Invalid number"));
    assert_eq!(convert_roman_numerals_to_int(&"I".to_string()), Ok(1));
    assert_eq!(convert_roman_numerals_to_int(&"II".to_string()), Ok(2));
    assert_eq!(convert_roman_numerals_to_int(&"III".to_string()), Ok(3));
    assert_eq!(convert_roman_numerals_to_int(&"IV".to_string()), Ok(4));
    assert_eq!(convert_roman_numerals_to_int(&"V".to_string()), Ok(5));
    assert_eq!(convert_roman_numerals_to_int(&"VI".to_string()), Ok(6));
    assert_eq!(convert_roman_numerals_to_int(&"VII".to_string()), Ok(7));
    assert_eq!(convert_roman_numerals_to_int(&"VIII".to_string()), Ok(8));
    assert_eq!(convert_roman_numerals_to_int(&"IX".to_string()), Ok(9));
    assert_eq!(convert_roman_numerals_to_int(&"X".to_string()), Ok(10));

    /* Random number */
    assert_eq!(convert_roman_numerals_to_int(&"XXXIX".to_string()), Ok(39));
    assert_eq!(convert_roman_numerals_to_int(&"CCXLVI".to_string()), Ok(246));
    assert_eq!(convert_roman_numerals_to_int(&"DCCLXXXIX".to_string()), Ok(789));
    assert_eq!(convert_roman_numerals_to_int(&"COUCOU".to_string()), Err("Invalid number"));

    /* Number with 0 */
    assert_eq!(convert_roman_numerals_to_int(&"CLX".to_string()), Ok(160));
    assert_eq!(convert_roman_numerals_to_int(&"CCVII".to_string()), Ok(207));

    /* Thousands numbers */
    assert_eq!(convert_roman_numerals_to_int(&"MCMXVIII".to_string()), Ok(1918));
    assert_eq!(convert_roman_numerals_to_int(&"MMXIV".to_string()), Ok(2014));

    /* Negative numbers */
    assert_eq!(convert_roman_numerals_to_int(&"-II".to_string()), Err("Invalid number: negative number does not exist in roman numerals"));
}


fn convert_int_to_roman_numerals(n: i32) -> Result<String, &'static str> {
    //let mut is_negative = false;
    let mut res = String::new();

    if n == 0 {
        return Err("Invalid number: number 0 does not exist in roman numerals");
    }

    if n < 0 {
        return Err("Invalid number: negative number does not exist in roman numerals");
    }

    let unit = n % 10;
    let tens = n % 100 - unit;
    let hundreds = n % 1000 - tens - unit;
    let thousands = n % 10000 - hundreds - tens - unit;

    match thousands {
        1000 => res.push_str("M"),
        2000 => res.push_str("MM"),
        3000 => res.push_str("MMM"),
        _ => res = res,
    }

    match hundreds {
        100 => res.push_str("C"),
        200 => res.push_str("CC"),
        300 => res.push_str("CCC"),
        400 => res.push_str("CD"),
        500 => res.push_str("D"),
        600 => res.push_str("DC"),
        700 => res.push_str("DCC"),
        800 => res.push_str("DCCC"),
        900 => res.push_str("CM"),
        _ => res = res,
    }
    
    match tens {
        10 => res.push_str("X"),
        20 => res.push_str("XX"),
        30 => res.push_str("XXX"),
        40 => res.push_str("XL"),
        50 => res.push_str("L"),
        60 => res.push_str("LX"),
        70 => res.push_str("LXX"),
        80 => res.push_str("LXXX"),
        90 => res.push_str("XC"),
        _ => res = res,
    }

    match unit {
        1 => res.push_str("I"),
        2 => res.push_str("II"),
        3 => res.push_str("III"),
        4 => res.push_str("IV"),
        5 => res.push_str("V"),
        6 => res.push_str("VI"),
        7 => res.push_str("VII"),
        8 => res.push_str("VIII"),
        9 => res.push_str("IX"),
        _ => res = res,
    }

    //println!("Thousands: {}", thousands);
    //println!("Hundreds: {}", hundreds);
    //println!("Tens: {}", tens);
    //println!("Unit: {}", unit);

    Ok(res.chars().collect())

}

fn convert_roman_numerals_to_int(s: &String) -> Result<i32, &'static str> {
    let my_string: Vec<char> = s.chars().collect();
    let mut res: i32 = 0;

    let mut unit = String::new();
    let mut tens = String::new();
    let mut hundreds = String::new();
    let mut thousands = String::new();

    let mut is_thousands_finish = false;
    let mut is_hundreds_finish = false;
    let mut is_tens_finish = false;

    if my_string[0] == '-' {
        return Err("Invalid number: negative number does not exist in roman numerals");
    }

    for i in 0 .. s.len() {
        let c = my_string[i];

        if c == 'M' {
            if is_thousands_finish {
                hundreds.push(c);
            }
            else {
                thousands.push(c);
            }
        }

        else if c == 'C' || c == 'D' {
            is_thousands_finish = true;
            if is_hundreds_finish {
                tens.push(c);
            }
            else {
                hundreds.push(c);
            }
        }

        else if c == 'X' || c == 'L' {
            is_hundreds_finish = true;
            if is_tens_finish {
                unit.push(c);
            }
            else {
                tens.push(c);
            }
        }

        else if c == 'I' || c == 'V' {
            is_tens_finish = true;
            unit.push(c);
        }

        else {
            return Err("Invalid number");
        }
    }

    println!("Thousands : {}", thousands);
    println!("Hundreds : {}", hundreds);
    println!("Tends : {}", tens);
    println!("Unit : {}", unit);

    match thousands.as_str() {
        "M" => res += 1000,
        "MM" => res += 2000,
        "MMM" => res += 3000,
        _ => res = res,
    }

    match hundreds.as_str() {
        "C" => res += 100,
        "CC" => res += 200,
        "CCC" => res += 300,
        "CD" => res += 400,
        "D" => res += 500,
        "DC" => res += 600,
        "DCC" => res += 700,
        "DCCC" => res += 800,
        "CM" => res += 900,
        _ => res = res,
    }

    match tens.as_str() {
        "X" => res += 10,
        "XX" => res += 20,
        "XXX" => res += 30,
        "XL" => res += 40,
        "L" => res += 50,
        "LX" => res += 60,
        "LXX" => res += 70,
        "LXXX" => res += 80,
        "XC" => res += 90,
        _ => res = res,
    }

    match unit.as_str() {
        "I" => res += 1,
        "II" => res += 2,
        "III" => res += 3,
        "IV" => res += 4,
        "V" => res += 5,
        "VI" => res += 6,
        "VII" => res += 7,
        "VIII" => res += 8,
        "IX" => res += 9,
        _ => res = res,
    }

    Ok(res)
}
