fn main() {

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("'{}' 的長度為 {}。", s1, len);

    // change(&s1);
    
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("'{}' 的長度為 {}。", s2, s2.len());

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // 這行會報錯，因為不能同時有兩個可變參考
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 離開作用域，所以建立新的參考也不會有問題
    let r2 = &mut s;

    let mut s3 = String::from("hello");
    let r3 = &s3; // 沒問題
    let r4 = &s3; // 沒問題
    // let r5 = &mut s; // 很有問題！
    // println!("{}, {}, and {}", r3, r4, r5);
    println!("{}, {}", r3, r4);
    // 變數 r3 和 r4 將不再使用
    let r5 = &mut s3; // 沒問題
    println!("{}", r5);

    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize { // s 是個 String 的參考
    s.len()
} // s 在此離開作用域，但因為它沒有它所指向的資料的所有權
  // 所以不會被釋放掉

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// } // 這行會報錯，因為 some_string 是不可變的參考

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}