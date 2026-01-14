
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