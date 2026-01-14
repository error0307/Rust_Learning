use futures::executor::block_on;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

// ⚠️ 学習用コード：
// このコードはFuture::pollの動作を理解するためのデモンストレーションです。
// 実務的な非同期処理では以下の問題があります：
// - poll()内でthread::sleep()を使用するとランタイム全体をブロック
// - 他のタスクが実行できず、非同期の利点が失われる
// - 実用的には async/awaitで tokio::time::sleep().await を使用すべき

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