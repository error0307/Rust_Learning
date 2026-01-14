use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let flag = Arc::new(Mutex::new(false)); // 共有カウンタの作成
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