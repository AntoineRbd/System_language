use std::fmt::Error;

macro_rules! ok_or_return {
    ($a:ident($($b:tt)*))=>{
        {
            match $a($($b)*) {
                Ok(value)=>value,
                Err(err)=>{
                    return Err(err);
                }
            }
        }
    };
}

type R = Result<i32, String>;

fn return_error() -> R {
    Err("Error".to_owned())
}

fn return_success() -> R {
    Ok(42)
}

fn test1() -> R {
    ok_or_return!(return_success());
    println!("Test Passed!");
    Ok(22)
}

fn test2() -> R {
    ok_or_return!(return_error());
    println!("Test Failed!");
    Ok(22)
}

fn test3() -> R {
    ok_or_return!(return_success(), return_error());
    println!("Test Failed!");
    Ok(22)
}

fn main() {
    test1();
    test2();
}
