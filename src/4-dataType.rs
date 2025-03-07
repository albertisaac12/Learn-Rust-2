fn main() {
    /*
        range -(2**(n-1)) to (2**(n-1))-1 for signed integers
        range 0 to (2**n)-1 for unsigned integer
    
     */

    // tuples can hold any kind of data

    let tup: (i32,f64,char) = (1,1.1,'c');

    let(x,y,z) = tup;

    // to access the elements inside the tuple use the . operator with index starting from 0

    println!("{}",tup.0);
    println!("{}",tup.1);
    println!("{}",tup.2);
    
}