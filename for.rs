fn main() {
    let a = [1,2,3,4,5,6];
    for each(a) |&e| {
        io::println(fmt!("%d", e));
    }
}

fn each(v: &[int], op: &fn(v: &int) -> bool) {
    let mut i = 0;
    while i < v.len() {
        op(&v[i]);
        i += 1;
    }
}
