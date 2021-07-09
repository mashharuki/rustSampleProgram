use std::fmt;

// 列挙型変数MyErrorを用意する。
enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

/**
 * MyErrorにDisplayトレイトを適用させる。
 */
impl fmt::Display for MyError {
    // fmt関数
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Num(cause) => write!(f, "Parse Error {}", cause),
        }
    }
}

/**
 * ファイルから数字を取得する関数
 */
fn get_int_from_file() ->Result<i32, MyError> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e))
}

/**
 * メイン関数
 */
fn main() {
    // get_int_from_file関数の結果がマッチしているかをチェックする。
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => match e {
            MyError::Io(cause) => println!("I/O Error: {}", cause),
            MyError::Num(cause) => println!("Parse Error {}", cause),
        },
    }
}