// ライブラリをインポートする。
use clap::{App, Arg};

/**
 * メイン関数
 */
fn main() {
    // Appインスタンスを作成する。
    let matches = App::new("My RPN program")
                        .version("1.0.0")
                        .author("Haruki Kondo")
                        .about("Super awesome sample RPN calculator")
                        .arg(
                            Arg::new("formula_file")
                                .about("Formulas written in RPN")
                                .value_name("FILE")
                                .index(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("verbose")
                                .about("Sets the level of verbosity")
                                .short('v')
                                .long("verbose")
                                .required(false),
                        )
                        .get_matches();

    // パターンにマッチしているかチェックする
    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified? :{}", verbose);
}
