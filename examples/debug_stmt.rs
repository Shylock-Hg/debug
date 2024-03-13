use debug::debug;

fn add(x: i32, y: i32) -> i32 {
    debug! {
        x + y
    }
}

fn main() {
    println!("Sum of 1 and 2 is {}.", add(1, 2));
}
