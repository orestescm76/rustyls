use std::fs::{read_dir, ReadDir};
fn main() {
    let res_dir = read_dir(".");
    match res_dir {
        Ok(dir) => print_dir(dir),
        Err(_) => panic!(),
    }
    println!("\x1b[32mHola, mundo!\x1b[0m");
}
fn print_dir(dir: ReadDir) {
    for file in dir {
        let entry = file.expect("fallo de lectura");
        let file_n = entry.file_name().into_string().unwrap_or_default();
        println!("{}", file_n);
    }
}