#[derive(Debug)] // 這讓我們可以顯示每個州
enum UsState {
    Alabama,
    Alaska,
    // --省略--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("最大值被設為 {}", max),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("最大值被設為 {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => println!("此 25 美分所屬的州為 {:?}!", state),
        _ => count += 1,
    }
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("此 25 美分所屬的州為 {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);

    let coin = Coin::Quarter(UsState::Alaska);
    println!("{:?}", describe_state_quarter(coin));

    let coin = Coin::Quarter(UsState::Alabama);
    println!("{:?}", describe_state_quarter_v2(coin));

    let coin = Coin::Nickel;
    println!("{:?}", describe_state_quarter_v3(coin));
    
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸運幣！");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("此 25 美分所屬的州為 {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {println!("添加一頂花哨的帽子！")}
fn remove_fancy_hat() {println!("移除花哨的帽子！")}
fn move_player(num_spaces: u8) {println!("移動玩家 {} 步！", num_spaces)}
fn reroll() {println!("重新擲骰子！")}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter_v2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter_v3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}