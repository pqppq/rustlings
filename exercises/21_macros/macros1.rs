// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

#[macro_export]
macro_rules! cec {
    ( $( $x:expr ), *) => {
        {
            let mut temp_vec = Vec::new();
            // ここがマクロ展開される
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
