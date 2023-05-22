pub fn string_push_str() -> String {
    let mut s1: String = String::from("Hello");
    let slice: &str = ", World!";
    s1.push_str(slice);
    s1
}

pub fn string_concat_str() -> String {
    let s1: String = String::from("Hello");
    let slice: &str = ", World!";
    let s2: String = s1 + slice;
    s2
}

pub fn string_format_str() -> String {
    let s1: String = String::from("Hello");
    let slice: &str = ", World!";
    let s2 = format!("{}{}", s1, slice);
    s2
}
