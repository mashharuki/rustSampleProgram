// ライブラリをインポートする。
use clap::Clap;
use std::fs::File;
use std::io::{ BufRead, BufReader, stdin };

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

// 構造体RpnCalculator
struct RpnCalculator(bool);

// RpnCalculatorの関数を実装する。
impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }
    // 計算関数
    pub fn eval(&self, formula: &str) -> i32 {
        0
    }
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
        // 標準入力に対応する
        let stdin = stdin();
        // 入力をロックする。
        let reader = stdin.lock();
        // run関数を呼び出す。
        run(reader, opts.verbose);
    }
}

/**
 * run関数
 */
fn run<R: BufRead>(reader: R, verbose: bool) {
    // RpnCalculator型のインスタンスを生成
    let calc = RpnCalculator::new(verbose);
    // 1行ずつファイルの内容を読み込む
    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", line);
    }
}
