use clap::{Parser, ArgAction};
#[derive(Parser)]
#[command(
    version = "1.0.0", 
    author = "Keisuke", 
    about = "An example of clap argument parsing"
)]

struct Args {
    #[arg(short, long, action = ArgAction::SetTrue, help = "Enable verbose mode")]
    verbose: bool, // verboseオプション 
    #[arg(short, long, help = "Specify a file")]
    file: Option<String>, // fileオプション
}

fn main() {
    let args = Args::parse(); // 引数の解析
    if args.verbose { // verboseオプションのチェック
        println!("Verbose mode enabled");
    }
    match args.file { // fileオプションのチェック
        Some(filename) => println!("File: {}", filename),
        None => println!("No file specified"),
    }
}