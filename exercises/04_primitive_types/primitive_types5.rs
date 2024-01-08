// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    // tupleのdestructuringはmatchを使う
    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html
    match cat {
        (name, age) => println!("{} is {} years old.", name, age),
    }
}
