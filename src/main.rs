fn main() {
    let mut ignore_case = "hello world";
    if !env::var("IGNORE_CASE").is_ok() {
        ignore_case = "cannot hello the world";
    }
    println!("{0}", ignore_case);
}
