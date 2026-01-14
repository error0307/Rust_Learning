use clap::{ArgAction, Command, arg};

fn main() {
    let matches = Command::new("My RPN program")
        .version("1.0.0")
        .author("Keisuke Ota")
        .about("Super Awesome Sample RPN Calculator")
        .arg(arg!([FILE] "Formulas Written in RPN").required(false))
        .arg(arg!(-v --verbose ... "Sets the level of verbosity").action(ArgAction::SetTrue))
        .get_matches();
        
    match matches.get_one::<String>("FILE") {
        Some(file) => println!("File Specified: {}", file),
        None => println!("No file specified"),
    }

    let verbosity = matches.get_flag("verbose");
    println!("Is verbosity specified?: {}", verbosity);
}
