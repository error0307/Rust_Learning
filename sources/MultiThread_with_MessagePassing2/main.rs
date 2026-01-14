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