struct Example {
    value: i32,
}

fn main() {
    let x = 5;
    let y = &x;

    println!("{}", x);  // ✅ OK
    println!("{}", y);  // ✅ OK (y is &i32)
    println!("{}", *y); // ✅ Dereference needed to get i32

    let e = Example { value: 10 };
    let r = &e;

    println!("{}", r.value);  // ✅ No `*` needed (Rust auto-derefs)
    println!("{}", (*r).value); // ✅ Also works, but unnecessary
}
