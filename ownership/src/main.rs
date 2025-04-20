fn main() {

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 將字面值加到字串後面
    println!("{}", s); // 這會印出 `hello, world!`


    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有權轉移到 s2，s1 不再有效
    // println!("{}", s1); // 這行會導致編譯錯誤，因為 s1 的所有權已經轉移到 s2
    println!("{}", s2); // 這行會正常執行，因為 s2 擁有 s1 的所有權


    let mut s3 = String::from("hello");
    s3 = String::from("ahoy"); // 這行將字串 "ahoy" 的所有權轉移到 s3
    println!("{}", s3); // 這行會正常執行，因為 s3 現在擁有新的字串的所有權


    let s4 = String::from("hello");
    let s5 = s4.clone(); // clone() 方法會創建一個新的字串，並將其所有權轉移到 s5
    println!("{}", s4); // 這行會正常執行，因為 s4 和 s5 都擁有各自的字串
    println!("{}", s5); // 這行會正常執行，因為 s5 擁有自己的字串的所有權


    let mut x = 5;
    let y = x;
    x = 6; // 這行會正常執行，因為 x 是一個整數類型的值，並且是 Copy 類型
    println!("{}", x); // 這行會正常執行，因為 x 現在是 6
    println!("{}", y); // 這行會正常執行，因為 y 是一個整數類型的值，並且是 Copy 類型


    let s = String::from("hello");  // s 進入作用域
    takes_ownership(s);             // s 的值進入函式
                                    // 所以 s 也在此無效
    // println!("{}", s);              // 這行會導致編譯錯誤，因為 s 的所有權已經轉移到函式裡
    let x = 5;                      // x 進入作用域
    makes_copy(x);                  // x 本該移動進函式裡
                                    // 但 i32 有 Copy，所以 x 可繼續使用
    println!("{}", x);              // 這行會正常執行，因為 x 還是有效的


    let s6 = gives_ownership();         // gives_ownership 移動它的回傳值給 s6
    println!("{}", s6);                // 這行會正常執行，因為 s6 擁有 gives_ownership 的回傳值
                                        // 這行會印出 "你的字串"
    let s7 = String::from("哈囉");     // s7 進入作用域
    let s8 = takes_and_gives_back(s7);  // s7 移入 takes_and_gives_back
                                        // 該函式又將其回傳值移到 s8
    // println!("{}", s7);            // 這行會導致編譯錯誤，因為 s7 的所有權已經轉移到函式裡
    println!("{}", s8);                // 這行會正常執行，因為 s8 擁有 takes_and_gives_back 的回傳值
                                        // 這行會印出 "哈囉"

    let s9 = String::from("hello");
    let (s10, len) = calculate_length(s9);
    // println!("{}", s9); // 這行會導致編譯錯誤，因為 s9 的所有權已經轉移到函式裡
    println!("'{}' 的長度為 {}。", s10, len);


} // x 在此離開作用域，接著是 s。但因為 s 的值已經被移動了
  // 它不會有任何動作

  // s8 在此離開作用域並釋放
  // s7 已被移走，所以沒有任何動作發生
  // s6 離開作用域並釋放

fn takes_ownership(some_string: String) { // some_string 進入作用域
    println!("{}", some_string);
} // some_string 在此離開作用域並呼叫 `drop`
  // 佔用的記憶體被釋放
fn makes_copy(some_integer: i32) { // some_integer 進入作用域
    println!("{}", some_integer);
} // some_integer 在此離開作用域，沒有任何動作發生


fn gives_ownership() -> String {             // gives_ownership 會將他的回傳值
                                             // 移動給呼叫它的函式
    let some_string = String::from("你的字串"); // some_string 進入作用域
    some_string                              // 回傳 some_string 並移動給
                                             // 呼叫它的函式
}
// 此函式會取得一個 String 然後回傳它
fn takes_and_gives_back(a_string: String) -> String { // a_string 進入作用域
    a_string  // 回傳 a_string 並移動給呼叫的函式
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 回傳 String 的長度

    (s, length)
}