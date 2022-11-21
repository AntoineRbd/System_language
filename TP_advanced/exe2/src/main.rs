macro_rules! print_and_execute {
    ($($x:expr), *) => {
        $(
            println!("expr:\n{}\nres:\n{}\n", stringify!($x), $x);
         )*
    };
}


fn main() {
    print_and_execute!(3, 34, "coucou", {let x = 34;  x}, { 
        struct Foo {a: i32}
        let i = Foo { a: 34 };
        i.a
    });
}
