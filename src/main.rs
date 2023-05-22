use bench_string_concat::{string_concat_str, string_format_str, string_push_str};

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
