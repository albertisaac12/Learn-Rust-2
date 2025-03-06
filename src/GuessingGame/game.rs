use std::io;
use rand::Rng; 
use std::cmp::Ordering;
/*  
    from the rand package use the Rng trait ("Note: In rust you need to bring in the trait into scope if you want to use the function from any Type that implements that trait"); 
    Best example is the Rng trait the reason we are bringing in the Rng trait into scope is to let rust know that the rand::thread_rng() implements the Rng trait to be specific the gen_range(); function

    actual flow => rand::thread_rng().gen_range(1..=100);
    now the rand::thread_rng() returns a random number generator (which maybe a struct) That generator implements the Rng trait, for rust to understand it implements Rng we bring the Rng trait into scope now we finally call the gen_range();
*/

/*
    form std library import the cmp module and from cmp module fetch Ordering enum; 

    pub enum Ordering {
        Less,
        Equal,
        Greater,
    }

*/
fn main() {
    println!("Guess the number! ");
    println!("Please input your guess.");

    
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("The secrect number is  {secret_number}");
    
    loop {
    let mut guess = String::new(); // mutable variable
    println!("Please Enter a Guess: ");
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    /*from the io module use the stdin() associated function that returns a Handle of the Stdin(A struct), on that struct call the read_line method which then returns a result type which either resolves to Ok or Err */
    println!("You guessed: {}",guess);

    // convert the guess to number
    /*
        We learn a couple of things below 1st we overload the variable we declare the guess again but this time its a u32 and rust is smart enough to understand the different guess types.
    */
    let guess:u32 = match guess.trim().parse() { 
        Ok(num) => num,
        Err(_) => continue,
    };

    /*
        The cmp method compares two values and can be called on anything that can be compared.
        It takes a reference to whatever you want to compare with: here it’s comparing guess to converted_num
     */
    match guess.cmp(&secret_number) {
        Ordering::Equal => {
            println!("You Win");
            break;
        }
        Ordering::Greater=> println!("Too big"),
        Ordering::Less => println!("To Less")
    }
}

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