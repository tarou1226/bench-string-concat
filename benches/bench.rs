// ベンチマークランタイムに必要な機能を有効化する。
#![feature(test)]

// ベンチマークランタイムを使用するためにextern crateを追加する。
extern crate test;

// ベンチマークランタイムのBencherをインポートする。
use test::Bencher;

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

#[bench]
fn bench_string_push_str(b: &mut Bencher) {
    b.iter(|| {
        string_push_str();
    });
}

#[bench]
fn bench_string_concat_str(b: &mut Bencher) {
    b.iter(|| {
        string_concat_str();
    });
}

#[bench]
fn bench_string_format_str(b: &mut Bencher) {
    b.iter(|| {
        string_format_str();
    });
}