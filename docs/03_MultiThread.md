# Rust学習記録その3 - マルチスレッドについて
Rustにおけるマルチスレッド処理の基礎について学んだ内容を記録します。

## Rustにおけるマルチスレッドの概念
マルチスレッド処理とは、複数のスレッドを同時に実行することで、プログラムの並行性とパフォーマンスを向上させる技術です。
プログラムの実行中、性能要求やユーザビリティの向上のために、複数の処理を同時に実行したい場合があります。Rustでは、標準ライブラリを通じて、適切な安全性を確保したうえで、マルチスレッド処理を簡単に実装できます。

## そもそもスレッドとは？
スレッドは、プロセス内で実行される軽量な実行単位です。 

![](https://storage.googleapis.com/zenn-user-upload/7e933194d78d-20251219.png)

その特徴は以下の通りです：
* コンテキストスイッチにより、プロセス内で複数の処理を疑似的に並列で実行可能
* 構造化された仕組みを用いて、スレッド間の通信が可能 → **メッセージパッシング**
* 同じプロセス内のメモリ空間を共有するため、データの共有が容易 → **共有メモリ**

## Rustにおけるスレッドの作成と管理
Rustでは、標準ライブラリの`std::thread`モジュールを使用してスレッドを作成および管理できます。
以下に、基本的なスレッドの作成方法を示します：

* `std::thread`モジュールを使用：`use std::thread;`
* スレッドの生成: `thread::spawn`関数を使用して新しいスレッドを生成します。
* スレッドの実行: スレッド内で実行したい処理をクロージャとして渡します。 
* スレッドの終了待機: `join`メソッドを使用して、スレッドの終了を待機します。


**クロージャ:**　Rustにおける無名関数の一種で、環境から変数をキャプチャして使用できる関数オブジェクトです。クロージャは、`|引数| 式`の形式で定義され、関数と同様に引数を受け取り、値を返すことができます。

```rust
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| { // SubThreadの作成
        for i in 1..5 {
            println!("SubThread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..5 {
        println!("MainThread: {}", i);
        thread::sleep(Duration::from_millis(700)); // 並列実行を確認するために少しずらす
    }

    handle.join().unwrap(); // スレッドの終了を待機
}
```
実行結果：
```
MainThread: 1
SubThread: 1
SubThread: 2
MainThread: 2
SubThread: 3
MainThread: 3
SubThread: 4
MainThread: 4
```

このように、MainThreadとSubThreadが並列に実行されていることが確認できます。 
ただし、スレッドはその終了タイミングが不明なため、変数のライフタイムがスレッドのライフタイムより短くなる可能性があり、キャプチャしようとするとコンパイルエラーとなることがあります。

```rust
use std::thread;
fn main() {
    let data = "Hello, World!";
    let handle = thread::spawn(|| { // コンパイルエラーになる例
        println!("SubThread: {}", data); // dataの所有権が移動しないためエラー
    });

    handle.join().unwrap(); // スレッドの終了を待機
}
```
* コンパイルエラー例:
```
error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
 --> src/main.rs:4:32
  |
4 |     let handle = thread::spawn(|| { // コンパイルエラーになる例
  |                                ^^ may outlive borrowed value `data`
5 |         println!("SubThread: {}", data); // dataの所有権が移動しないためエラー
  |                                   ---- `data` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:4:18
  |
4 |       let handle = thread::spawn(|| { // コンパイルエラーになる例
  |  __________________^
5 | |         println!("SubThread: {}", data); // dataの所有権が移動しないためエラー
6 | |     });
  | |______^
help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
  |
4 |     let handle = thread::spawn(move || { // コンパイルエラーになる例
  |                                ++++

For more information about this error, try `rustc --explain E0373`.
```

そのため、`move`キーワードを使用して、変数の所有権をスレッドに移動させます。

```rust
use std::thread;
fn main() {
    let data = "Hello, World!";
    let handle = thread::spawn(move || { //　moveキーワードを使用して所有権を移動
        println!("SubThread: {}", data); // dataの所有権がスレッドに移動
    });

    handle.join().unwrap(); // スレッドの終了を待機
}
```
実行結果：
```
SubThread: Hello, World!
```


## スレッド間の通信とデータ共有
Rustでは、スレッド間の通信とデータ共有を安全に行うために、以下の2つの主要な方法が提供されています：
1. **共有メモリ**: `std::sync::Arc`（Atomic Reference Counted）と`std::sync::Mutex`を組み合わせて使用し、複数のスレッドが共有されたメモリを読み書きすることで、データの共有が可能になります。
                    `Arc`はデータの所有権を共有し、`Mutex`は同時アクセスを制御します。
2. **メッセージパッシング**: `std::sync::mpsc`を使用し、メッセージキューを介してスレッド間でデータを送受信が可能になります。

## 共有メモリによるデータ共有の例

共有メモリを使用して、複数のスレッド間でフラグを共有し、フラグの状態に応じて処理を切り替える例を示します。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let flag = Arc::new(Mutex::new(false)); // 共有フラグの作成
    let mut handles = vec![];

    for i in 0..5 {
        let flag = Arc::clone(&flag); // Arcをクローンしてスレッドに渡す
        let handle = thread::spawn(move || {
            let mut flag_locked = flag.lock().unwrap(); // 一度だけロック
            if *flag_locked == false {
                *flag_locked = true; // フラグをオン
                println!("Thread{}: FLAG ON", i);
            } else {
                *flag_locked = false; // フラグをオフ
                println!("Thread{}: FLAG OFF", i);
            }
        }); 
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap(); // 全てのスレッドの終了を待機
    }
}
```

上記のコードについて説明します：
* `Arc<Mutex<bool>>`を使用して、複数のスレッドで共有されるフラグを作成。
    * `Arc`は複数の所有者を持つことを可能にする型。C++の`shared_ptr`に似ています。
    * `Mutex`は排他制御を提供し、同時アクセスを防ぐためのロック機構を提供します。
* 各スレッドは`Arc::clone`を使用して`flag`の参照を取得
    * `clone`を用いて参照カウントを増やし、所有権を共有します。
* スレッド内で`lock`メソッドを使用して`Mutex`をロックし、フラグの状態を確認および更新します。
    * `lock`メソッドは、`Mutex`をロックし、ロックが取得できるまで待機します。

上記のコードの実行結果です：
```
Thread0: FLAG ON
Thread1: FLAG OFF
Thread4: FLAG ON
Thread3: FLAG OFF
Thread2: FLAG ON
```

`flag`へアクセスする順序は毎回異なりますが、`Mutex`により同時アクセスが防がれ、フラグの状態が正しく更新されていることが確認できます。



## メッセージパッシングによるスレッド間通信の例

次にメッセージパッシングを使用して、スレッド間でデータを送受信する例を示します。

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // チャンネルの作成

    let handle = thread::spawn(move || {
        let messages = vec![
            String::from("Hello,"),
            String::from("How are you?"),
        ];

        for msg in messages {
            println!("Sending: {}", msg);
            tx.send(msg).unwrap(); // メッセージの送信
            thread::sleep(std::time::Duration::from_millis(1000)); // 送信間隔を調整
        }
    });

    for received in rx {
        println!("Received: {}", received); // メッセージの受信
    }

    handle.join().unwrap(); // スレッドの終了を待機
}
```
上記のコードについて説明します：()`を使用して、送信側のエンドポイント（`tx`）と受信側のエンドポイント（`rx`）のペアを作成します。
* `tx`をスレッドに移動し、`send()`メソッドを使用してメッセージを送信します。
* メインスレッドでは、`rx`を使用してメッセージを受信します。
  * `rx`はイテレータとして動作し、すべての`tx`がドロップされるまでメッセージを受信し続けます。

実行結果：
実行結果:
```
Sending: Hello,
Received: Hello,
Sending: How are you?
Received: How are you?
```

この例の場合、サブスレッドがメッセージを送信し、メインスレッドがそれを受信していますが、逆にメインスレッドが送信し、サブスレッドが受信することも可能です。チャンネルを2つ作成し、各スレッドに送信と受信のエンドポイントを渡すことで、双方向通信も実現できます。

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx_to_subthread, rx_from_main) = mpsc::channel(); // メインからサブスレッドへのチャンネル
    let (tx_to_main, rx_from_subthread) = mpsc::channel(); // サブスレッドからメインへのチャンネル

    let handle = thread::spawn(move || {
        let messages = vec![
            String::from("Hello,"),
            String::from("How are you?"),
        ];

        for msg in messages {
            println!("SubThread Sending: {}", msg);
            tx_to_main.send(msg).unwrap(); // メッセージの送信
            thread::sleep(std::time::Duration::from_millis(1000)); // 送信間隔を調整
        }

        drop(tx_to_main); // チャンネルを閉じる

        // メインスレッドの送信を待つ
        for received in rx_from_main {
            println!("SubThread Received: {}", received); // メッセージの受信
        }
    });

    // サブスレッドからのメッセージを受信しながら、
    // 受信完了後にメッセージを送信する
    for received in rx_from_subthread {
        println!("MainThread Received: {}", received); // メッセージの受信
    }

    // ここでrx_from_subthreadは閉じられている

    let messages = vec![
            String::from("I'm fine,"),
            String::from("Thank you!"),
    ];

    for msg in messages {
        println!("MainThread Sending: {}", msg);
        tx_to_subthread.send(msg).unwrap(); // メッセージの送信
        thread::sleep(std::time::Duration::from_millis(1000)); // 送信間隔を調整
    }

    drop(tx_to_subthread); // チャンネルを閉じる
    
    handle.join().unwrap(); // スレッドの終了を待機
}
```


実行結果：
```
SubThread Sending: Hello,
MainThread Received: Hello,
SubThread Sending: How are you?
MainThread Received: How are you?
MainThread Sending: I'm fine,
SubThread Received: I'm fine,
MainThread Sending: Thank you!
SubThread Received: Thank you!
```

:::message
この双方向通信の例では、メインスレッドとサブスレッドの両方で受信待ちが存在しますが、実際のアプリケーションではデッドロックに注意が必要です。適切な同期機構やタイムアウトを導入することで、デッドロックのリスクを軽減できます。
:::

## 実際の活用例

マルチスレッド処理は、以下のようなシナリオで特に有用です：
* **並列計算**：大量のデータ処理や数値計算を複数のスレッドで分散して実行することで、処理時間を短縮できます。
* **I/O待機の最適化**：ネットワーク通信やファイル操作などのI/O待機時間を他のスレッドで処理することで、全体のパフォーマンスを向上させます。
* **リアルタイム処理**：ユーザーインターフェースの応答性を維持しながら、バックグラウンドでデータ処理を行うことができます。

## まとめ

Rustのマルチスレッド処理は、安全性と効率性を兼ね備えた強力な機能です。標準ライブラリを活用することで、スレッドの作成、管理、通信、データ共有が容易に行えます。Rust特有のルールとも相まって、難しい部分もありますが、理解できれば、より並行性の高いアプリケーションを構築できるようになります。
ぜひマスターしたいですね。