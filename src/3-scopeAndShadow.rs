fn main() {
    let x = 1;
    let x = x+10;

    {
        let x= x+14;
        println!("{x}");
    }

    println!("{x}");

    // works totaly fine
    let mut spaces = "   ";
    let spaces = spaces.len();

    /*will throw error
        let mut spaces = "   ";
        spaces = spaces.len();
    */

    let arr =[3; 5];


    let s: &str = "hiii";

    let m =s;

    println!("{s}");    
}