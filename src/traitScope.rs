struct Dog;

trait Speak {
    fn speak(&self);
}

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn main() {
    let my_dog = Dog;
    my_dog.speak();
}
