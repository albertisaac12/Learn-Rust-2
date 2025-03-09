#[derive(Debug)] // println! with debug trait only makes a borrow
struct Rectangle {
    witdh:u32,
    length:u32
}


impl Rectangle {
    fn area(&mut self) -> u32 {
        self.witdh+=5;
        self.length*self.witdh
    }

    fn wickedArea(self)-> u32 {
        self.length*self.witdh
    }

    fn square(dimension: u32) -> Self {
        Self {
            witdh: dimension,
            length: dimension
        }
    }
}

// &self is short for 

fn main() {
    let mut r1 = Rectangle {
        witdh : 10,
        length :15
    };

    let area = r1.area();

    println!("{area}");
    println!("{r1:#?}"); // will work fine commented because using the r1 with self in the next piece of code
    
    let area = r1.wickedArea();
    println!("{area}");
    // println!("{r1:#?}"); // will throw an error 

    let mut r2 = Rectangle::square(10);

    r2.witdh = 100;

    println!("{}",r2.area());
}