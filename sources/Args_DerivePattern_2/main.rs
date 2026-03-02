use clap::{Parser, ArgAction};
#[derive(Parser)]
#[command(
    version = "1.0.0", 
    author = "Keisuke", 
    about = "An example of clap argument parsing"
)]

struct Args {
    #[arg(name = "NUMBER", help = "Enter a number")]
    num: i32, // 数値を直接入力 
    #[arg(short, long, help = "Specify a file")]
    file: Option<String>, // fileオプション
}

fn main() {
    let args = Args::parse(); // 引数の解析
    println!("Number: {}", args.num); // 数値の表示
    match args.file { // fileオプションのチェック
        Some(filename) => println!("File: {}", filename),
        None => println!("No file specified"),
    }
}