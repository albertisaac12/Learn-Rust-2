use core::slice;

fn main() {
    let bytes = "0x1".as_bytes();

    // let mut s = String::from(
    //     "hello world"
    // );

    // let word = first_word(&s);

    // s.clear();

    let mut s = String::from(
        "hello world"
    );

    let slice =  &s[0..6]; // slices return string literals

    let word = modified_first_word(&s);


    
    s.clear(); // here error comes because .clear(); takes a mutable reference to the variable s

    println!("{word}"); // until and unless you use the borrow or a variable assigned to borrow it wont work

}

fn first_word(s: &String)-> usize {
    let bytes = s.as_bytes(); // this will convert it to array of bytes

    for (i,&item) in bytes.iter().enumerate() { // the first element that enumerate returns is the index and second is a reference to the item.
        if item == b' ' {
            return i;
        }

    }
    s.len()
}

fn modified_first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // this will convert it to array of bytes

    for (i,&item) in bytes.iter().enumerate() { // the first element that enumerate returns is the index and second is a reference to the item.
        if item == b' ' {
            return &s[0..i];
        }

    }
    &s[..]
}

fn even_better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // this will convert it to array of bytes

    for (i,&item) in bytes.iter().enumerate() { // the first element that enumerate returns is the index and second is a reference to the item.
        if item == b' ' {
            return &s[0..i];
        }

    }
    &s[..]
}


// So ASCII and bytes can be compared


// slices can also be used on arrays

// fn return_a_array_slice() -> &[i32; 4] {
//     let a = [1, 2, 3, 4, 5];

//     let slice = &a[1..3];
// }

fn return_array() -> [i32; 4] {
    [1, 2, 3, 4]
}
