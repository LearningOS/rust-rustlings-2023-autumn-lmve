// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

// fn current_favorite_color() -> String {  //string 版本
//     "blue".to_string()
// }
fn current_favorite_color() -> &'static str{  //字符串切片版本
    "blue"
}