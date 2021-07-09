use core::num;

use anyhow::{ Context, Result };

/**
 * ファイルから数字を取得する関数
 */
fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).with_context(|| format!("failed to read string from {}", path))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .context("failed to parse string")
}

/**
 * メイン関数
 */
fn main() {
    // get_int_from_file関数の結果がマッチしているかをチェックする。
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}