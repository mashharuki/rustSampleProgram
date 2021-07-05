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
        // 空白で区切る
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        // 内部計算用の関数を呼び出す
        self.eval_inner(&mut tokens)
    }
    // 内部計算関数
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        // Vec型のインスタンスを生成
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                // 四則演算用の記号であるかチェックする。
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }
            // この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        if stack.len() == 1{
            stack[0]
        } else {
            panic!("invalid syntax")
        }
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
