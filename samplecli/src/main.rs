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
    
}
