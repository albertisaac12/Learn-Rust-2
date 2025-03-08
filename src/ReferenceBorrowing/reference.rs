fn main() {

    let s1 = String::from("hello");

    let len = calculate_length(&s1); // passing the reference
    
    println!("{s1}");
    println!("The length of the string is {len}.");

    let mut s = String::from("meow");

    change(&mut s); // this function is borrowing the variable s

    println!("{s}");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3); // will throw error as s was initially borrowed as an immutable reference 


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
    //(*some_string).push_str(", world"); // behind the scenes
}

