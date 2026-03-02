use clap::{ArgAction, Command, arg};
fn main() {
    let matches = Command::new("ArgsSample") //Commandオブジェクトの作成
        .version("1.0.0") // バージョン情報
        .author("Keisuke") // author情報
        .about("An example of clap argument parsing") // プログラムの説明
        .arg(arg!(-v --verbose "Enable verbose mode").action(ArgAction::SetTrue)) // verboseオプション
        .arg(arg!(-f --file <FILE> "Specify a file").required(false)) // fileオプション
        .get_matches(); // 引数の解析
    
    if matches.get_flag("verbose") { // verboseオプションのチェック
        println!("Verbose mode enabled");
    }

    if let Some(filename) = matches.get_one::<String>("file") { // fileオプションの取得
        println!("File: {}", filename);
    }
}