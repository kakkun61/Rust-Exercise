fn main() {
    let mut one = 1;
    let mut two = 2;
    io::println(fmt!("%d %d", one, two));
    swap(&mut one, &mut two);
    io::println(fmt!("%d %d", one, two));
}

fn swap(a: &mut int, b: &mut int) {
    let t = *b;
    *b = *a;
    *a = t;
}
