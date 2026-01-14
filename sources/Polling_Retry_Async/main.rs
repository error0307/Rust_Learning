// ✅ 実務的な非同期スリープを使った実装
// このコードはasync/awaitとtokio::time::sleepを使用し、
// 非同期ランタイムをブロックしません。

use tokio;

// リソース取得関数（実務的な非同期実装）
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
                    // ✅ 非同期スリープ（ランタイムをブロックしない）
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
}

// 疑似的なリソース取得関数
fn get_resource(request_id: u32, call_count: u8) -> Result<u32, String> {
    if call_count < 2 {
        Err(format!("Retry {}", call_count))
    } else {
        Ok(request_id + 100)
    }
}

#[tokio::main]
async fn main() {
    println!("=== 実務的な非同期スリープを使った実装 ===");
    
    match request_resource(0).await {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // 複数タスクを同時実行（他のタスクが実行中に待機できる）
    println!("\n=== 複数タスクの同時実行（I/O中に他のタスクが実行可能） ===");
    
    let task1 = request_resource(1);
    let task2 = request_resource(2);
    
    // この間に他のタスクが実行される
    let (result1, result2) = tokio::join!(task1, task2);
    
    println!("Task1: {:?}", result1);
    println!("Task2: {:?}", result2);
}

/* 
■ poll()内でthread::sleep()を使う場合の問題：
  
  Thread1 が poll() 実行中
  └─ thread::sleep(1秒) ← ⚠️ Thread1 全体がブロック
     └─ 他のタスク（Thread1上で動作）が実行できない
     
■ tokio::time::sleep().await の場合：

  Thread1 が poll() 実行中
  └─ tokio::time::sleep().await ← 制御をランタイムに譲る
     └─ その間に他のタスクが実行される（同じThread1上で）
     └─ 1秒後に poll() が再開される
     
つまり、await は「待機中に他のタスクに制御を譲る」仕組みです。
*/
