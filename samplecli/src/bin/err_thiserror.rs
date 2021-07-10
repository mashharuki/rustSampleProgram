use thiserror::Error;

// 列挙型変数MyErrorを用意する。
#[derive(Error, Debug)]
enum MyError {
    #[error("failed to read string from {0}")]
    ReadError(String),
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),
}

/**
 * ファイルから数字を取得する関数
 */
fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|_| MyError::ReadError(path.into()))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(MyError::from)
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