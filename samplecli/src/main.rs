// ライブラリをインポートする。
use clap::Clap;
use std::fs::File;
use std::io::{ BufRead, BufReader };

#[derive(Clap, Debug)]
#[clap(
    name= "My RPN program",
    version = "1.0.0",
    author = "Haruki Kondo",
    about = "Super swesome sample RPN calculator"
)]

// 構造体Opts
struct Opts {
    // sets the level of vervosity
    #[clap(short, long)]
    verbose: bool,
    // Formula written in RPN
    #[clap(name = "FILE")]
    furmula_file: Option<String>,
}

/**
 * メイン関数
 */
fn main() {
    let opts = Opts::parse();
    // 計算ロジックを追加する。
    if let Some(path) = opts.furmula_file {
        // ファイル名を取得する
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        // run 関数を呼び出す。
        run(reader, opts.verbose);
    } else {
        // ファイルを指定しなかった場合
        println!("No file is specified");
    }
}

/**
 * run関数
 */
fn run(reader: BufReader<File>, verbose: bool) {
     // 1行ずつファイルの内容を読み込む
     for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
