use std::io;
fn main() {
    println!("Guess the number! ");
    println!("Please input your guess.");

    let mut guess = String::new(); // mutable variable


    io::stdin().read_line(&mut guess).expect("Failed to read line");
    /*from the io module use the stdin() assocaited function that returns a Handle of the Stdin(A struct), on that struct call the read_line method which then returns a result type which either resolves to Ok or Err */
    println!("You guessed: {}",guess);

}

// Important things to learn generating a random number along with taking the input from the console.

/*

    io => input/output library into scope. The io library comes from the standard library, Knowns as std
    By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude
    If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io library provides you with a number of useful features, including the ability to accept user input.
    variables are immutable by default, meaning once we give the variable a value, the value won’t change

    The :: syntax in the ::new line indicates that new is an associated function of the String type.
    (Basically string is a struct with some functions within :: is used to invoke them)
    An associated function is a function that’s implemented on a type, in this case String

    The & indicates that this argument is a reference, 
    which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.


    Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
    Result’s variants are Ok and Err. 
    The Ok variant indicates the operation was successful, and it contains the successfully generated value. 
    The Err variant means the operation failed, and it contains information about how or why the operation failed.

    Result has an expect method that you can call. If this instance of Result is an Err value, 
    expect will cause the program to crash and display the message that you passed as an argument to expect.

    Crate is a collection of rust source code files.
*/