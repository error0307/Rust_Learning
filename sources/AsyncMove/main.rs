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