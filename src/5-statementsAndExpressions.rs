fn main() {
    // statement => are instructions that perform some action and do not return a value.
    // expressions => evaluate to a resultant value. Lets look at some examples.

    let x = 10; // statement but the value 10 is a expression

    // expressions can be a part of a statement

    // Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression

    let x = {
        let y =3;
        y+1
    };

    let f = plus_five(x);

    println!("{f}");


}

fn plus_five(x:i32) -> i32 {
    x+5
    // x+5; will become a statement so semicolon at the end changes it from a expression to a statement
}

