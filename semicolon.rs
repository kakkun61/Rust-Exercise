fn main() {
    let a;
    if true {
        a = 10;
    } else {
        a = 11;
    }
    let b = if true {
        20
    } else {
        21
    };
    io::println(fmt!("a: %d", a));
    io::println(fmt!("b: %d", b));
}

