fn main() {
    let mut i = 0;
    loop {
        io::println(
                match i {
                    x if i % 15 == 0 => "fizzbuzz",
                    x if i %  5 == 0 => "buzz",
                    x if i %  3 == 0 => "fizz",
                    _                => fmt!("%d", i)
                }
        );
        i += 1;
        if i == 20 {
            break;
        }
    }
}
/*
fizzbuzz.rs:9:40: 10:17 error: mismatched types: expected `&'static str` but found `~str` (str storage differs: expected ~ but found &'static )
*/
