## bench-string-concat
文字列の結合する方法をベンチした。

### 問題
Rustの文字列を結合する方法は複数あるが、どれが早いのかを気になった。

### 文字連結方法
- Stringにstrを追加する
- Stringとstrを連結させ、新しいStringを作成する
- Sringとstrをformatマクロに入れる

### 解決策
考えられる方法をベンチマークする。

### 結果
```bash
# bash or terminal
# cargoのbenchコマンドがnightlyのみ対応しているので、変更しています。
$ rustup default nightly

$ cargo bench
running 3 tests
test bench_string_concat_str ... bench:          36 ns/iter (+/- 0)
test bench_string_format_str ... bench:          99 ns/iter (+/- 4)
test bench_string_push_str   ... bench:          36 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out; finished in 0.94s
```