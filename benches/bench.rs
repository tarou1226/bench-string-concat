// ベンチマークランタイムに必要な機能を有効化する。
#![feature(test)]

// ベンチマークランタイムを使用するためにextern crateを追加する。
extern crate test;

// ベンチマークランタイムのBencherをインポートする。
use bench_string_concat::*;
use test::Bencher;

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
