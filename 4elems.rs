fn main() {
    
}

struct FourElems<T> {
    one: T,
    two: T,
    three: T,
    four: T
}

fn each<T>(fe: &FourElems<T>, op: &fn(a: T) -> ()) {
    op(fe.one);
    op(fe.two);
    op(fe.three);
    op(fe.four);
}
/*
4elems.rs:14:7: 14:9 error: use of partially moved value: `fe`
4elems.rs:14     op(fe.two);
                    ^~
4elems.rs:13:7: 13:13 note: field of `fe` moved here because the field has type 'a, which is moved by default (use `copy` to override)
*/
