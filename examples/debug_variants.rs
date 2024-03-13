use debug::debug_field;

debug_field! {
    enum Test {
        #[allow(dead_code)]
        X(i32),
        #[debug]
        Y(i32),
    }
}

fn main() {
    let t = Test::Y(2);
    if let Test::Y(y) = t {
        println!("y: {}", y);
    }
}
