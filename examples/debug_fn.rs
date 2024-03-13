use debug::debug;

debug! {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn main() {
    let sum = add(1, 2);
    println!("Sum of 1 and 2 is {}", sum);
}
