/**
 * テキストファイルから数字を取得する関数
 */
fn get_int_from_file() -> Result<i32, String> {
    // ファイルの読み込む
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

    num_str
        .trim()  // 文字列の前後の空白を削除する。
        .parse::<i32>()  // &strをi32型に変換する。
        .map(|t| t * 2)
        .map_err(|e| e.to_string())
}

/**
 * メイン関数
 */
fn main() {
    // パターンマッチをチェックする。
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}