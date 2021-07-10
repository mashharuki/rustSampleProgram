// ライブラリをインポートする。
use clap::Clap;
use std::fs::File;
use std::path::PathBuf;
use std::io::{ BufRead, BufReader, stdin };
use anyhow::{ bail, ensure, Context, Result };

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
    furmula_file: Option<PathBuf>,
}

// 構造体RpnCalculator
struct RpnCalculator(bool);

// RpnCalculatorの関数を実装する。
impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }
    // 計算関数
    pub fn eval(&self, formula: &str) -> Result<i32> {
        // 空白で区切る
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        // 内部計算用の関数を呼び出す
        self.eval_inner(&mut tokens)
    }
    // 内部計算関数
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        // Vec型のインスタンスを生成
        let mut stack = Vec::new();
        // エラーが起きた場所を特定するための変数
        let mut pos = 0; 

        while let Some(token) = tokens.pop() {
            // 加算する。
            pos += 1;
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;
                // 四則演算用の記号であるかチェックする。
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }
            // この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        // スタックの要素数が1以外ならエラー
        ensure!(stack.len() == 1, "invalid syntax");
        Ok(stack[0])
    }
}

/**
 * メイン関数
 */
fn main() -> Result<()> {
    let opts = Opts::parse();
    // 計算ロジックを追加する。
    if let Some(path) = opts.furmula_file {
        // ファイル名を取得する
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        // run 関数を呼び出す。
        run(reader, opts.verbose)
    } else {
        // 標準入力に対応する
        let stdin = stdin();
        // 入力をロックする。
        let reader = stdin.lock();
        // run関数を呼び出す。
        run(reader, opts.verbose)
    }
}

/**
 * run関数
 */
fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    // RpnCalculator型のインスタンスを生成
    let calc = RpnCalculator::new(verbose);
    // 1行ずつファイルの内容を読み込む
    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }
    Ok(())
}

/**
 * テストコード
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        // RpnCalculator型の変数を用意する。
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);
        // 四則演算のテスト
        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        // RpnCalculator型の変数を用意する。
        let calc = RpnCalculator::new(false);
        assert!(calc.eval("").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}