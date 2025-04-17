fn main() {
    let number = 3;

    if number < 5 {
        println!("條件為真");
    } else {
        println!("條件為否");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("數字結果為：{number}");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("升空！！！");
}
