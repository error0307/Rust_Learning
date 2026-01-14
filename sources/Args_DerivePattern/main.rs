use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "My RPN Program",
    version = "1.0.0",
    author = "Keisuke Ota",
    about = "Super awesome sample RPN calculator"
)]

struct Cli {
    /// Sets the level of verbosity
    #[arg(short, long)]
    verbose: bool, //これはフラグなのでbool型

    /// Sets the path
    #[arg(short, long, default_value = "/home/keisuke/")]
    path: String, //これは必須引数なのでString型

    /// Number
    #[arg(short, long)]
    number: Option<i32>,

    /// Formulas written in RPN
    #[arg(short, long)]
    file: Option<String>,
}

fn main(){
    let cli = Cli::parse();

    match cli.file {
        Some(file) => println!("File Specified: {}", file),
        None => println!("No file specified."),
    }
    match cli.number {
        Some(n) => println!("Number Specified: {}", n),
        None => println!("No number specified."),
    }

    println!("Is verbosity specified? : {}", cli.verbose);
    println!("Path specified: {}", cli.path);
}
