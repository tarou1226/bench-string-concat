fn string_push_str() -> String {
    let mut s1: String = String::from("Hello");
    let slice: &str = ", World!";
    s1.push_str(slice);
    s1
}

fn string_concat_str() -> String {
    let s1: String = String::from("Hello");
    let slice: &str = ", World!";
    let s2: String = s1 + slice;
    s2
}

fn string_format_str() -> String {
    let s1: String = String::from("Hello");
    let slice: &str = ", World!";
    let s2 = format!("{}{}", s1, slice);
    s2
}

fn main() {
    println!("Hello, World!");
    // 関数の動作確認
    let hello1 = string_push_str();
    let hello2 = string_concat_str();
    let hello3 = string_format_str();
    println!("{}", hello1);
    println!("{}", hello2);
    println!("{}", hello3);
}