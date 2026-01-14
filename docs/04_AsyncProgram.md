# Rust学習記録その4 - 非同期プログラミングについて
前回のマルチスレッドに続き, 今回は非同期プログラミングについて学んだ内容を記録します。
いやあ面白くなってきた。

## Rustにおける非同期プログラミング
非同期プログラミングはその名の通り、通常は現在の処理が完了してから次の処理に移る、同期的な処理とは異なり、処理の完了を待たずに次の処理に移ることができるプログラミング手法です。  

Rustでは、非同期プログラミングをサポートするために、`async`/`await`構文と`Future`トレイトが提供されています。これらを用いて、非同期処理をタスクとして定義し、ランタイムによって管理・スケジューリングされます。

主な利点は以下の通りです：

* **効率的なリソース利用**：I/O待機中に他のタスクを実行することで、リソースを有効活用できます。
* **応答性の向上**：UIがブロックされずにバックグラウンド処理を続行できます。
* **スケーラビリティ**：多数の同時接続を軽量に処理できるため、サーバーアプリケーションに適しています。

## マルチプロセス, マルチスレッドとの違い
非同期的な処理は、マルチプロセスや、前回説明したマルチスレッドと似た目的を持っていますが、いくつかの重要な違いがあります。

| 特性 | マルチプロセス | マルチスレッド | 非同期プログラミング |
|------|----------------|----------------|----------------------|
|メモリ| 独立したメモリ空間 | 共有メモリ空間 | 共有メモリ空間 |
|コンテキスト切替| 高コスト | 低コスト | 非常に低コスト |
|オーバーヘッド| 高い | 低い | 非常に低い |
|同期| プロセス間通信が必要 | ロックやミューテックスが必要 | 非同期関数とFutureで管理 |
|CPU利用| 複数CPUコアを活用可能 | 複数CPUコアを活用可能 | 単一スレッドでも効率的に利用可能 |

重要な部分は, 非同期プログラミングは, 単一スレッドでも効率的にCPUを利用できる点です. これは, 非同期処理がI/O待ちなどのブロッキング操作を回避し, 他のタスクを進行させることができるためです. そのため, 軽量なタスク管理が可能であり, コンテキスト切替のオーバーヘッドも非常に低く抑えられます.

使い分けとしては, CPU集約的な処理や, 独立したリソース管理が必要な場合はマルチプロセスやマルチスレッドが適していますが, I/O待ちが多いアプリケーションや, 軽量なタスク管理が求められる場合は非同期プログラミングが有効です.


## 非同期プログラミングのユースケース
### FutureトレイトとPollによる非同期処理の実装
まずは, FutureトレイトとPollを用いた非同期処理の実装例を示します.
下記は, あるリソースを取得する非同期処理を, リトライのロジックを含めて実装したものです。

* リトライのロジック
```rust
use futures::executor::block_on;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

struct RequestResource {
    request_id: u32,
    retry_count: u8,  // リトライカウンタ
}

impl RequestResource {
    fn new(request_id: u32) -> Self {
        RequestResource {
            request_id,
            retry_count: 0,  // 初期値は0
        }
    }
}

impl Future for RequestResource {
    type Output = u32;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // リソース取得を試行
        let result = get_resource(self.request_id, self.retry_count);
        
        match result {
            Ok(handler) => {
                println!("Success!"); 
                Poll::Ready(handler) // 成功した場合はReadyを返す
            }
            Err(e) => {
                self.retry_count += 1;
                if self.retry_count >= 3 {
                    println!("Max retry count reached. RetryOut!");
                    Poll::Ready(0)  // 3回リトライ後はRetryOut
                } else {
                    println!("{}", e);
                    cx.waker().wake_by_ref();
                    Poll::Pending // 再度ポーリングを要求
                }
            }
        }
    }
}

fn main() {
    let request_resource = RequestResource::new(0);

    let handler = block_on(request_resource);

    println!("Handler: {}", handler);
    
}

// 疑似的なリソース取得関数
fn get_resource(request_id: u32, call_count: u8) -> Result<u32, String> {
    if call_count < 2 {
        // 1. 2回目はエラー
        Err(format!("Retry {}", call_count))
    } else {
        // 3回目以降はOk
        Ok(request_id + 100)
    }
}

```

実行結果：
```
Retry 0
Retry 1
Success!
Handler: 100
```

### コード説明

`RequestResource`構造体に`Future`トレイトを実装し、リトライロジックを含めています。

* `poll`メソッド内で`get_resource`を呼び出し、結果に応じて処理を分岐
* 成功時：`Poll::Ready(handler)`を返却
* 失敗時で未満リトライ：`cx.waker().wake_by_ref()`で再度ポーリングを要求し`Poll::Pending`を返却
* リトライ上限到達：`Poll::Ready(0)`を返却

main関数で`block_on`を使用して非同期処理を実行しています。

## async/awaitを用いた非同期処理の実装

同じリトライロジックを`async`/`await`構文で実装すると、より直感的で簡潔になります。 

```rust

// リソース取得関数
async fn request_resource(request_id: u32) -> Result<u32, String> {
    let mut retry_count = 0;
    
    loop {
        match get_resource(request_id, retry_count) {
            Ok(handler) => {
                println!("Success!");
                return Ok(handler);
            }
            Err(e) => {
                retry_count += 1;
                if retry_count >= 3 {
                    println!("Max retry count reached. RetryOut!");
                    return Err("RetryOut".to_string());
                } else {
                    println!("{}", e);
                    // リトライ前に待機
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let handler = request_resource(0).await.map_err(|e| format!("Error: {}", e));
    println!("Handler: {:?}", handler);
}

// 疑似的なリソース取得関数
fn get_resource(request_id: u32, call_count: u8) -> Result<u32, String> {
    if call_count < 2 {
        Err(format!("Retry {}", call_count))
    } else {
        Ok(request_id + 100)
    }
}
```

実行結果：
```
Retry 0
Retry 1
Success!
Handler: 100
```

### コード説明

Futureトレイト実装より遥かに簡潔です：

* `async fn request_resource`で非同期関数を定義
* `loop`でリトライロジックを直感的に記述
* 成功時は`Ok(handler)`を返却、失敗時はリトライカウンタをインクリメント
* `#[tokio::main]`でランタイムを起動し、`.await`で非同期処理を実行


## async moveの使用法

`async move`は、非同期クロージャ内で外部変数の所有権を明示的に移動させるキーワードです。

通常の`async`ブロックでは外部変数を参照としてキャプチャしますが、`async move`を使用すると所有権そのものを移動させることができます。これにより、非同期処理が独立して実行でき、ライフタイムの問題を回避できます。

複数のリクエストIDに対して並行実行する例を示します：

```rust
use std::time::Duration;
use futures::future::join_all;

// リソース取得関数
async fn request_resource(request_id: u32) -> Result<u32, String> {
    let mut retry_count = 0;
    
    loop {
        match get_resource(request_id, retry_count) {
            Ok(handler) => {
                println!("Request {} - Success!", request_id);
                return Ok(handler);
            }
            Err(e) => {
                retry_count += 1;
                if retry_count >= 3 {
                    println!("Request {} - Max retry count reached. RetryOut!", request_id);
                    return Err("RetryOut".to_string());
                } else {
                    println!("Request {} - {}", request_id, e);
                    // リトライ前に待機（非同期スリープ）
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }
}

// async moveの例：複数のリクエストを並行実行
async fn request_multiple_resources() {
    let request_ids = vec![0, 1, 2];
    
    // async moveで外側の変数を所有権移動してFutureを作成
    let futures: Vec<_> = request_ids
        .into_iter()
        .map(|id| async move {
            println!("Processing request_id: {}", id);
            request_resource(id).await
        })
        .collect();
    
    // join_allで複数のFutureを並行実行
    // ⚠️ 重要：forループで逐次的に.awaitを呼ぶのではなく、
    // join_all()で全てのFutureを同時に実行させる
    let results = join_all(futures).await;
    
    for (index, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("Request {} Result: {}", index, value),
            Err(e) => println!("Request {} Error: {}", index, e),
        }
    }
}

#[tokio::main]
async fn main() {
    println!("\n=== Multiple requests with async move and join_all ===");    
    request_multiple_resources().await;

}

// 疑似的なリソース取得関数
fn get_resource(request_id: u32, call_count: u8) -> Result<u32, String> {
    if call_count < 2 {
        Err(format!("Retry {}", call_count))
    } else {
        Ok(request_id + 100)
    }
}
```

実行結果：
```
=== Multiple requests with async move and join_all ===
Processing request_id: 0
Request 0 - Retry 0
Processing request_id: 1
Request 1 - Retry 0
Processing request_id: 2
Request 2 - Retry 0
Request 0 - Retry 1
Request 1 - Retry 1
Request 2 - Retry 1
Request 0 - Success!
Request 1 - Success!
Request 2 - Success!
Request 0 Result: 100
Request 1 Result: 101
Request 2 Result: 102
```

### コード説明

`request_multiple_resources()`関数で複数のリクエストIDを並行処理しています：

* `into_iter()`でリクエストIDをイテレート
* `async move`クロージャ内で`id`変数の所有権を明示的に移動
* 各非同期処理が独立して実行され、ライフタイムの問題を回避
* `join_all`で全てのFutureを並行実行し、結果を収集

## まとめ
非同期プログラミングは, 効率的なリソース利用や応答性の向上, スケーラビリティの向上などの利点があります. Rustでは, `async`/`await`構文と`Future`トレイトを用いて、比較的簡単に非同期処理を実装可能だということがわかりました.
おそらく、この部分を理解すれば、あんなことやこんなことができるんじゃないかと想像が膨らみ、
ようやくRustによるプログラミングが面白いと感じられるようになるのではないかと思います。