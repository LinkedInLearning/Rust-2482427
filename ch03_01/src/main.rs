use std::{
    thread::{self, sleep, Thread},
    time::Duration,
};

// Utilité du borrow checker
fn main() {
    // Data race
    let mut ma_string = String::from("ceci est un test");
    let thread_1 = thread::spawn(move || {
        ma_string.push_str("!!!");
        sleep(Duration::from_millis(500));
    });
    let thread_2 = thread::spawn(move || {
        ma_string.push_str("???");
        sleep(Duration::from_millis(500));
    });

    thread_1.join();
    thread_2.join();

    println!("ma string {ma_string}");
}
