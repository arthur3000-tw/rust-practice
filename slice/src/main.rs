fn main() {
    
    let mut s = String::from("hello world");
    let word = first_word(&s); // word 取得數值 5
    println!("s: {}", s);
    s.clear(); // 這會清空 String，這就等於 ""
    // word 仍然是數值 5 ，但是我們已經沒有相等意義的字串了
    // 擁有 5 的變數 word 現在完全沒意義！

    let mut s2 = String::from("hello world");
    let word2 = first_word_v2(&s2); // word2 取得字串 "hello"
    // println!("s2: {}", s2);
    s2.clear(); // 這會清空 String，這就等於 ""
    // println!("第一個單字為：{}", word2); // 錯誤


    let my_string = String::from("hello world");
    // first_word 適用於 `String` 的切片，無論是部分或整體
    let word = first_word_v3(&my_string[0..6]);
    let word = first_word_v3(&my_string[..]);
    // first_word 也適用於 `String` 的參考，這等同於對整個 `String` 切片的操作。
    let word = first_word_v3(&my_string);
    let my_string_literal = "hello world";
    // first_word 適用於字串字面值，無論是部分或整體
    let word = first_word_v3(&my_string_literal[0..6]);
    let word = first_word_v3(&my_string_literal[..]);
    // 因為字串字面值本來就是切片
    // 沒有切片語法也是可行的！
    let word = first_word_v3(my_string_literal);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_v3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}