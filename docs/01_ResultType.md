# Rust学習記録その1 - Resultについて
Rustの学習を進める中で、エラーハンドリングのための重要な型である`Result`について躓く部分がありました。

備忘録として、記録しておきます。

## 前提
- Rustにおけるenum型の基本的な理解ができている

## std::result::Result
std::resultモジュールで定義されており、Preludeによって自動的にスコープに導入される列挙型。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- `Ok(T)`: 操作が成功した場合に返される値を含みます。
- `Err(E)`: 操作が失敗した場合に返されるエラー情報を含みます。

※"T", "E"はジェネリック型パラメータであり、使用者によって具体的な型が指定可能。

## Resultの使用例
以下は、`Result`を使用した簡単な例です。

```rust
// 2つの数値を割り算する関数
fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Division by zero")) // 0で割ろうとした場合はErrorとして文字列を返す
    } else {
        Ok(dividend / divisor) // 正常に割り算ができた場合は結果を数値で返す
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result), //パターンマッチングでOkの場合の処理
        Err(e) => println!("Error: {}", e), //パターンマッチングでErrの場合の処理
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result), //パターンマッチングでOkの場合の処理
        Err(e) => println!("Error: {}", e), //パターンマッチングでErrの場合の処理
    }
}
```

エラーハンドリングは、match式を使用して行われることが一般的ですが、`Result`にはもう少しスマートに実装できるよう、メソッドも用意されています。

## エラーハンドリングでよく使用されるメソッド
以下は、`Result`型を用いたエラーハンドリングでよく使われるメソッドの一覧です。

- `unwrap()`: `Ok`の場合は中の値を返し、`Err`の場合はパニック
```rust
let result: Result<i32, &str> = Ok(10);
let value = result.unwrap(); // valueは10

let error_result: Result<i32, &str> = Err("An error occurred");
let error_value = error_result.unwrap(); // ここでパニック
```

- `expect(msg: &str)`: `Ok`の場合は中の値を返し、`Err`の場合に指定したメッセージでパニック
```rust
let result: Result<i32, &str> = Err("An error occurred");
let value = result.expect("Failed to get value"); // "Failed to get value"でパニック
```

- `map<F, U>(self, op: F) -> Result<U, E>`: `Ok`の場合に関数`op`を適用し、新しい`Result`を返す。`Err`の場合はそのまま返す。
```rust
let result: Result<i32, &str> = Ok(10);
let new_result = result.map(|x| x * 2); // new_resultはOk(20)

let result2: Result<i32, &str> = Ok(10);

fn double(x: i32) -> i32 {
    x * 2
}

let new_result2 = result2.map(double); // これでも同じくOk(20)を返す
```

- `map_err<F, O>(self, op: F) -> Result<T, O>`: `Err`の場合に関数`op`を適用し、新しい`Result`を返す。`Ok`の場合はそのまま返す。
```rust
let result: Result<i32, &str> = Err("An error occurred");
let new_result = result.map_err(|e| format!("Error: {}", e)); // new_resultはErr("Error: An error occurred")

let result2: Result<i32, &str> = Err("An error occurred");

fn format_error(e: &str) -> String {
    format!("Error: {}", e)
}

let new_result2 = result2.map_err(format_error); // これでも同じくErr("Error: An error occurred")を返す
```

- `and_then<F, U>(self, op: F) -> Result<U, E>`: `Ok`の場合に関数`op`を適用し、新しい`Result`を返す。`Err`の場合はそのまま返す。(`map`と似ているが、`op`が`Result`を返す点が異なる。)
```rust
let result: Result<i32, &str> = Ok(10);
let new_result = result.and_then(|x| Ok(x * 2)); // クロージャを適用してOk(20)を返す

let result2: Result<i32, &str> = Ok(10);

fn func(x: i32) -> Result<i32, &str> {
    Ok(x * 2)
}

let new_result2 = result2.and_then(func); // これでも同じくOk(20)を返す
```

- `unwrap_or(default: T) -> T`: `Ok`の場合は中の値を返し、`Err`の場合は指定したデフォルト値を返す。
```rust
let result: Result<i32, &str> = Err("An error occurred");
let value = result.unwrap_or(0); // valueは0
```

- `unwrap_or_else<F>(self, op: F) -> T`: `Ok`の場合は中の値を返し、`Err`の場合は関数`op`を呼び出してその結果を返す。
```rust
let result: Result<i32, &str> = Err("An error occurred");
let value = result.unwrap_or_else(|_| -1); // valueは-1
```

## ちょっと応用のエラーハンドリング

* 早期リターン : 関数中で?演算子を使用し, エラー発生時は即座に`Err`を返す。

```rust
fn read_file_data(path: &str) -> Result<String, std::io::Error> {
    let read_data = std::fs::read_to_string(path)?; // エラーが発生した場合, 即座にErrを返す
    Ok(read_data)
}
```

* let else構文 : Rust 1.65以降で使用可能(になったらしい)。`Result`のパターンマッチングを簡潔に記述。

```rust
fn parse_number(input: &str) -> Result<i32, String> {
    let Ok(number) = input.parse::<i32>() else {
        return Err(format!("Failed to parse '{}' as number", input));
    };
    Ok(number)
}
```

* 複数のResultの連結 : `map_err`, `and_then`を使用して, 複数の`Result`を連結。

```rust
fn process_data(data: &str) -> Result<usize, String> {
    Ok(data.len())
}
fn read_and_process_file(path: &str) -> Result<usize, String> {
    std::fs::read_to_string(path)
        .map_err(|e| e.to_string()) // Errの場合は文字列に変換
        .and_then(|content| process_data(&content)) // Okの場合はprocess_dataを呼び出す
}
```

## Ok, Err以外のパターンを追加できるか?
Rustでは、継承の概念がないため、`Result`型自体を拡張して新しいパターンを追加することはできません。

システムによっては、複数のエラーパターンや状態を表現したい場合もあるかと思いますが、その場合、独自のenum型を定義する可能です。

```rust
// システムに特化したエラー型をenumで定義
enum SystemError {
    FatalError,
    RecoverableError,
    Warning,
}

fn perform_operation(condition: i32) -> Result<i32, SystemError> {
    match condition {
        1 => Err(SystemError::FatalError),
        2 => Err(SystemError::RecoverableError),
        _ => Ok(200),
    }
}
```

## おわりに
これまでのレガシーなシステムだと、関数戻り値の型は独自で定義され、細かな仕様の違いからバグの要因になることが多々ありましたが、Rustの`Result`型を使用することで、一貫性を持ったエラーハンドリングが可能になるのは非常に魅力的だと感じました。

理解必須ですね...

## 参考資料
https://doc.rust-lang.org/std/result/enum.Result.html

https://qiita.com/kerupani129/items/1e2c5f267081d0dba023

https://amzn.asia/d/hFXpmAg