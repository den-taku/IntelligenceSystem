fn main() {
    println!("Hello, world!");
    fn f(x: i32) -> i32 {
        f(x)
    }
    f(34);
}
