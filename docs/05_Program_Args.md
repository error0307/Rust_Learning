# Rust学習記録その5 - プログラム引数の解析について
前回の更新からだいぶ間が空いてしまいましたが, 気をとりなおして, 引き続きRustについてえっちらおっちら学んでいきたいと思います。
今回は、Rustでのユースケースが多いであろう、CLIアプリケーションの引数の解析について学んだ内容を記録します。


## std::env::argsによる引数の取得
Rustの標準ライブラリでは、`std::env::args`関数を使用してコマンドライン引数を取得できます。この関数は、プログラムの引数をイテレータとして返します。
```rust
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();　// 引数をVec<String>に収集
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }
}
```
実行結果:
```
cargo run a b c d 123
// 省略
Argument 0: target/debug/ArgsSample
Argument 1: a
Argument 2: b
Argument 3: c
Argument 4: d
Argument 5: 123
```
この例では、`env::args()`で取得した引数を`Vec<String>`に収集し、`enumerate()`でインデックスとともに表示しています。引数の最初の要素（インデックス0）はプログラム名です。

この方法はシンプルですが、オプション引数やファイルパスの入力など、複雑な実装を行う場合には手動で解析ロジックを実装する必要があります。

```Rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // 引数をVec<String>に収集
    
    let mut verbose = false; // --verboseオプションのフラグ
    let mut show_help = false; // --helpオプションのフラグ
    let mut filename = String::new(); // -fオプションのファイル名
    
    let mut i = 1;
    
    while i < args.len() {
        match args[i].as_str() {
            "--v" | "--verbose" => { // verboseオプション
                verbose = true;
                i += 1;
            }
            "--h" | "--help" => { // helpオプション
                show_help = true;
                i += 1;
            }
            "-f" | "--file" => { // fileオプション
                if i + 1 < args.len() {
                    filename = args[i + 1].clone(); // 次の引数をファイル名として取得
                    i += 2;  // 次のオプションまでスキップ
                } else {
                    eprintln!("Error: -f requires a filename");
                    return;
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                i += 1;
            }
        }
    }
    
    if show_help { // ヘルプ表示
        println!("Usage: app [OPTIONS]");
        println!("  --v, --verbose       Enable verbose mode");
        println!("  --h, --help          Show help");
        println!("  -f, --file <FILE>    Specify a file");
        return;
    }
    
    if verbose { // verboseモード
        println!("Verbose mode enabled");
    }
    
    if !filename.is_empty() { // ファイル名が指定されている場合
        println!("File: {}", filename);
    }
}
```

```実行例:
cargo run -- --h
//省略
Usage: app [OPTIONS]
  --v, --verbose       Enable verbose mode
  --h, --help          Show help
  -f, --file <FILE>    Specify a file

cargo run -- --v -f sample.txt
//省略
Verbose mode enabled
File: sample.txt
```

引数の処理だけで、かなりのコード量になってしまいました。オプションが増えるとさらに複雑になります。
この場合は、次に紹介する`clap`クレートを使用することが推奨されます。

## clapクレートによる引数解析
`clap`は、RustでCLIアプリケーションの引数解析を簡単に行うための強力なクレートです。`clap`を使用すると、オプションや引数の定義、ヘルプメッセージの自動生成などが非常に簡単になります。

さきほどの例を`clap`を使用して書き直してみます。

### builderパターンを使用した例
```rust
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
```

実行例:
```
cargo run -- --h
//省略
An example of clap argument parsing

Usage: ArgsSample [OPTIONS]

Options:
  -v, --verbose      Enable verbose mode
  -f, --file <FILE>  Specify a file
  -h, --help         Print help
  -V, --version      Print version

cargo run -- --v -f sample.txt
//省略
Verbose mode enabled
File: sample.txt
``` 

コード量を大幅に削減できました。
それぞれのコードの処理内容を説明していきます。

