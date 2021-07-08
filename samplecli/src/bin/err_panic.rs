/**
 * エラーハンドリング用の関数
 */
fn get_int_from_file() -> i32 {
    // 
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).expect("failed to open the file.");
    let ret = num_str.trim().parse::<i32>().expect("fail to parse string to a number.");
    // 2倍にする。
    ret * 2
}

/**
 * メイン関数
 */
fn main() {
    println!("{}", get_int_from_file());
}