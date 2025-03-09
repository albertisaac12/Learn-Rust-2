#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

impl UsState{
    fn existed_in(&self,year: u16) -> bool {
        match self {
            UsState::Alabama => year >=1819, 
            UsState::Alaska => year >= 1959
        }
    }
}

#[derive(Debug)] 
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let blah = Some(5);
    let mc:Option<i32> = None;

    matchEnum(&blah);
    matchEnum(&mc);

    let config_num = Some(3u8);

    if let Some(num) = config_num { // here since config_num is a u8 and has a Copy trait it works just fine
        println!("{num}");
    } else {
        println!("hiyaaaaaaaaaaaa");
    }

    println!("{config_num:?}"); // well
    println!("{config_num:?}");
    println!("{config_num:?}");
    println!("{config_num:?}");

    let config_num = Some(String::from("Hello"));

    if let Some(num) = config_num {  // Moves `num` out of `config_num`
        println!("{num}"); // Works fine, `num` now owns the string
    }

    // println!("{config_num:?}"); // ❌ ERROR: `config_num` was moved! 


    let c1 = Coin::Quarter(UsState::Alabama);

    let reply = value_in_cents(c1);
    println!("{reply}");
    // println!("{c1:?}"); // will throw a borrow of the moved value

    let c2 = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(blah) = c2 {
        println!("{blah:?}");
    }else {
        println!("meow");
    }

    let c3 = Coin::Quarter(UsState::Alaska);

    let bb = describe_state_quarter(c3);

    if let Some(str) = bb {
        println!("{str}")
    }else {
        println!("its nothing");    
    }

    let c4 = Coin::Quarter(UsState::Alabama);
    let cc = describe_state_quarter2(c4);

    if let Some(str) = cc {
        println!("{str}")
    }else {
        println!("its nothing");    
    }

}

fn matchEnum(blah: &Option<i32>) {
    match blah {
        Some(t) => {
            println!("{}",t+1);
        }
        None => {
            println!("its nothing");
        }
    }
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}


fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin { 
    if state.existed_in(1900) { // this works because state is a instance of UsState and not a type more like a state variable
            Some(format!("{state:?} is pretty old, for America!"))
        } else {    
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}


fn describe_state_quarter2(coin: Coin) -> Option<String> {
    let blah = if let Coin::Quarter(state) = coin {
        state // state is being assigned to the blah variable
    } else {
        return None;
    };

    if blah.existed_in(1900) {
        Some(format!("{blah:?} is pretty old, for America!"))
    } else {
        Some(format!("{blah:?} is relatively new."))
    }
}


