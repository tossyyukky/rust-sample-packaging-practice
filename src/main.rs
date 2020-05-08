use std::env;
use std::fs::File;
use std::io::BufReader;

use wordcount::count;

fn main() {
    // 必要な処理
    // 1. コマンドラインで指定された引数を読み込む
    // 2. 指定されたファイルを開く
    // 3. ファイルから1行ずつ読み込む
    // 4. その行を単語で分割する
    // 5. 出現した単語の出現頻度を数える

    // 1. コマンドラインで指定された引数を読み込む
    let filename = env::args().nth(1).expect("1 argument FILENAME reqiured");
    // 2. 指定されたファイルを開く
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    // 3. ファイルから1行ずつ読み込む
    let freqs = count(reader);
    println!("{:?}", freqs);

}
