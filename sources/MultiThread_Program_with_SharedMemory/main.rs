use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

// トランザクションデータ構造体
#[derive(Clone, Debug)] // CloneとDebugトレイトを実装
struct Transaction {
    id: u32,
    timestamp: SystemTime,
}

impl Transaction {
    fn new(id: u32) -> Transaction {
        Transaction {
            id,
            timestamp: SystemTime::now(),
        }
    }
}

fn queue_register(queue: &TransactionQueue, transaction: Transaction) {
    let mut queue_lock = queue.lock().unwrap(); // Mutexをロックしてキューにアクセス
    queue_lock.push(transaction); // キューにトランザクションを追加
    println!("[Client] Registered Transaction ID: {}", transaction.id);
}

// トランザクションキューの型定義
//   * Arc: マルチスレッドでメモリ共有するための型
//   * Mutex: 排他制御を行うための型
//   * Vec<Transaction>: トランザクションデータのベクター
type TransactionQueue = Arc<Mutex<Vec<Transaction>>>; 

fn main() {
    // TransactionQueue型のキューを作成（スレッドセーフなキュー）
    let queue: TransactionQueue = Arc::new(Mutex::new(Vec::new()));

    //Arcを使って各スレッドにキューの所有権を共有

    // メインスレッドのキューコピー
    let main_queue = Arc::clone(&queue);
    // クライアントスレッドのキューコピー
    let client_queue = Arc::clone(&queue);
    // ハンドラースレッドのキューコピー
    let handler_queue = Arc::clone(&queue); //Cloneを使ってキューの所有権を共有

    // 各スレッドでキューのメモリを共有しながら処理を実行

    // ハンドラースレッド：キューからデータを取得して処理
    let _handler_thread = thread::spawn(move || { // moveクロージャでキューの所有権をスレッドに移動
        loop {
            thread::sleep(Duration::from_secs(2));
            
            // キューから最初のトランザクションを取得
            let mut queue_lock = handler_queue.lock().unwrap(); // Mutexをロックしてキューにアクセス
            if !queue_lock.is_empty() { // is_empty: Vectorが空かどうかを確認するメソッド
                let transaction = queue_lock.remove(0); // remove(0): Vectorから指定した要素を削除して取得するメソッド
                drop(queue_lock); // すでにキューからデータを取得したので、早期にロックを解放
                
                println!(
                    "[Handler] Process Transaction - ID: {}, Timestamp: {:?}",
                    transaction.id, transaction.timestamp
                );
            }
        }
    });

    // メインスレッド：キューデータを定期的に出力
    let _main_thread = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(3));
            
            let queue_lock = main_queue.lock().unwrap();
            println!("[Main] Current Queue Size: {}", queue_lock.len());
            if !queue_lock.is_empty() {
                println!("[Main] Queue Contents:");
                for (idx, tx) in queue_lock.iter().enumerate() {
                    println!("  [{}] Transaction ID: {}", idx, tx.id);
                }
            }
        }
    });

    // クライアントスレッド：トランザクションデータを生成してキューに登録
    let client_thread = thread::spawn(move || {
        let mut transaction_id = 1u32;
        
        loop {
            thread::sleep(Duration::from_secs(1));
            
            let transaction = Transaction::new(transaction_id);
            
            queue_register(&client_queue, transaction);
            
            transaction_id += 1;
            
            // 10回でスレッド終了
            if transaction_id > 10 {
                println!("[Client] Client thread finished");
                break;
            }
        }
    });

    // クライアントスレッドの完了を待機
    client_thread.join().unwrap();
    
    // メインとハンドラースレッドは無限ループなので、適当に時間待機
    thread::sleep(Duration::from_secs(15));
    
    println!("[Main] Program finished");
}

