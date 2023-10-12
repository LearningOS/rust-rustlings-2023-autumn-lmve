// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.



// TODO: Complete this use statement
// 方法1
// use std::time::SystemTime;
// use std::time::UNIX_EPOCH;
//方法二
//使用嵌套路径将相同的项在一行中引入作用域，指定路径的相同部分，接着：：{各自不同路径}
use std::time::{SystemTime,UNIX_EPOCH};
fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
