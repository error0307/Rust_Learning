# Rust学習記録その5 - プログラム引数の解析について
前回の更新からだいぶ間が空いてしまいました。。。  
気をとりなおして, 引き続きRustについてえっちらおっちら学んでいきたいと思います。
今回は、Rustでのユースケースが多いであろう、CLIアプリケーションの引数の解析について学んだ内容を記録します。


## std::env::argsによる引数の取得
Rustの標準ライブラリでは、`std::env::args`関数を使用してコマンドライン引数を取得できます。この関数は、プログラムの引数をイテレータとして返します。
```rust
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect(); // 引数をVec<String>に収集
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

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // 引数をVec<String>に収集
    
    let mut verbose = false; // --verboseオプションのフラグ
    let mut show_help = false; // --helpオプションのフラグ
    let mut filename = String::new(); // -fオプションのファイル名
    
    let mut i = 1;
    
    while i < args.len() {
        match args[i].as_str() {
            "-v" | "--verbose" => { // verboseオプション
                verbose = true;
                i += 1;
            }
            "-h" | "--help" => { // helpオプション
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
        println!("  -v, --verbose       Enable verbose mode");
        println!("  -h, --help          Show help");
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

```bash
$ cargo run -- -h
Usage: app [OPTIONS]
  -v, --verbose       Enable verbose mode
  -h, --help          Show help
  -f, --file <FILE>    Specify a file

$ cargo run -- -v -f sample.txt
Verbose mode enabled
File: sample.txt
```

引数の処理だけで、かなりのコード量になってしまいました。  
オプションが増えるとさらに複雑になります。

:::message
このような複雑な引数解析には、`clap`クレートの使用が推奨されます。
:::

## clapクレートによる引数解析
`clap`は、RustでCLIアプリケーションの引数解析を簡単に行うための強力なクレートです。`clap`を使用すると、オプションや引数の定義、ヘルプメッセージの自動生成などが非常に簡単になります。

さきほどの例を`clap`を使用して書き直してみます。

### Builderパターンを使用した例

Builderパターンとは、GoFのデザインパターンの一つで、複雑なオブジェクトの構築を簡単にするためのパターンです。`clap`では、`Command`オブジェクトを構築する際にBuilderパターンが使用されており、メソッドチェーンを利用して引数やオプションを定義できます。

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

実行例：

```bash
$ cargo run -- -h
An example of clap argument parsing

Usage: ArgsSample [OPTIONS]

Options:
  -v, --verbose      Enable verbose mode
  -f, --file <FILE>  Specify a file
  -h, --help         Print help
  -V, --version      Print version
```

```bash
$ cargo run -- -v -f input.txt
Verbose mode enabled
File: input.txt
```

コード量を大幅に削減できました。
それぞれのコードの処理内容を説明していきます。

* `Command::new("ArgsSample")`でclapのコマンドオブジェクトを作成します。
* `version`, `author`, `about`でバージョン情報、作者情報、プログラムの説明を設定します。
* `arg!`マクロを使用して、引数やオプションを定義します。`arg!`は使用文字列から`Arg`を作成するマクロで、短オプション・長オプション・値名・複数指定・ヘルプを簡潔に定義できます。
* `action(ArgAction::SetTrue)`で、`--verbose`オプションが指定された場合にフラグを立てるように設定します。
* `required(false)`で、`--file`オプションが必須でないことを指定します。
* `get_matches()`で引数の解析を行い、結果を`matches`に格納します。

### deriveマクロを使用した例
`clap`では、deriveマクロを使用して、構造体に引数の定義を直接記述することもできます。  
これにより、さらにコードが簡潔になります。

```rust
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
```

実行例：

```bash
$ cargo run -- -h
An example of clap argument parsing

Usage: running_env [OPTIONS]

Options:
  -v, --verbose      Enable verbose mode
  -f, --file <FILE>  Specify a file
  -h, --help         Print help
  -V, --version      Print version
```

```bash
$ cargo run -- -v -f input.txt
Verbose mode enabled
File: input.txt
```

こちらも、それぞれのコードの処理内容を説明していきます。
* `#[derive(Parser)]`で、構造体に対してclapの引数解析機能を有効にします。
* `#[command(...)]`で、コマンド全体のメタ情報を設定します。
* `#[arg(...)]`で、構造体のフィールドに対して引数の定義を行います。
* `short`で短オプション、`long`で長オプションを指定します。フィールド名の1文字目が短オプション、フィールド名が長オプションとして自動的に設定されます。
* `action(ArgAction::SetTrue)`で、`--verbose`オプションが指定された場合にフラグを立てるように設定します。
* `Option<String>`型のフィールドを使用して、`--file`オプションが指定された場合にファイル名を格納できるようにします。
* `Args::parse()`で引数の解析を行い、結果を`args`に格納します。
* `args.verbose`でverboseオプションの状態をチェックし、`args.file`でfileオプションの値を取得します。  


また、deriveマクロを使用した場合、引数の値が構造体のフィールドの型へ自動的に変更されます。  

```rust
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
```
実行例：

```bash
$ cargo run -- 60 -f input.txt
Number: 60
File: input.txt
```
この例では、`num`フィールドが`i32`型で定義されているため、コマンドライン引数から数値を直接入力することができます。引数の値は自動的に`i32`型に変換され、`args.num`で数値を取得できます。これにより、引数の型変換のコードを書く必要がなくなり、さらにコードが簡潔になります。また、数値以外の値が入力された場合には、エラーが発生し、適切なエラーメッセージが表示されます。

実行例（エラーケース）：

```bash
$ cargo run -- num -f input.txt
error: invalid value 'num' for '<NUMBER>': invalid digit found in string

For more information, try '--help'.
```

## まとめ
今回は、Rustでのプログラム引数の解析について、標準ライブラリを使用した方法と、`clap`クレートを使用した方法の両方を紹介しました。  
`clap`を使用することで、引数の定義や解析が非常に簡単になり、コードの可読性も向上します。特に、deriveマクロを使用することで、さらにコードが簡潔になり、引数の型変換も自動的に行われるため、非常に便利です。  
基本的にどのOSSも`clap`を使用して引数の解析を行っていることが多いので、RustでCLIアプリケーションを開発する際には、必ず必要な知識となるかと思います。