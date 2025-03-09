enum IpAddr{
    v4(String),
    v6(String)
}

struct b {
    witdh:u32,
    length:u32
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    Stu(b)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting"),
            _=> println!("done")

        }
    }

    fn cc() {
        println!("Cc");
    }
}


fn main() {
    let v4 = IpAddr::v4(String::from("8.8.8.8"));
    let v6 = IpAddr::v4(String::from("::1"));

    Message::cc();

    let printQuit = Message::Quit;
    printQuit.call();
}

