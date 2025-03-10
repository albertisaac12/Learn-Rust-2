struct Device {
    name: String,
    status: String,
}

fn main () {
    let nickname: Option<String> = Some("Rustacean".to_string());
    nicknameExisists(&nickname);

    let v1 = Ok(10);
    let v2 = Err("Failed to get value".to_string());

    println!("{}", double_value(v1)); // Should print 20
    println!("{}", double_value(v2)); // Should print -1


    let d1 = Device {
        name: "Thermostat".to_string(),
        status: "online".to_string(),
    };
    let d2 = Device {
        name: "Camera".to_string(),
        status: "offline".to_string(),
    };

    isOnline(&d1);
    isOnline(&d2);

    let my_string = String::from("hello");
    
    // Dereferencing String converts it into &str
    let my_str: &str = &*my_string; // manuel dereferencing

    let my_str_2:&str = &my_string; // this will work automatic dereferencig
    
    println!("{}", my_str);  // Prints "hello"

    let some_option = Some("hello");

    let Some(_value) = some_option else {
        println!("No value found!");
        return;
    };
}

fn nicknameExisists(meow: &Option<String>) {

    if let Some(str) = meow {
        if str!=" " {
            println!(" Nickname Exisits");
        }
    }

}

fn double_value(r:Result<i32,String>) -> i32 {
    
    let value = if let Ok(m) = r {
        m*2
    } else {
        return -1;
    };

    value

}


fn isOnline(d: &Device) {

    let _blah = "online".to_string();

    if let Device {status: mv, ..} = d {
        if d.status == *_blah {
            println!("Device is active");
        }
    }

}