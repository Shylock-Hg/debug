use debug::debug_field;

debug_field! {
    struct Test {
        x: i32,
        #[debug]
        y: i32,
    }
}

fn main() {
    let t = Test { x: 1, y: 2 };
    println!("x: {}, y: {}", t.x, t.y);
}
