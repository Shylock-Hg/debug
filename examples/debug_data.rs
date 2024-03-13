use debug_code::debug;

debug! {
    struct Test {
        pub i: i32,
    }
}

fn main() {
    let t = Test { i: 1 };
    println!("{}", t.i);
}
