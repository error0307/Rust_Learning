use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap(); // IPアドレスとポートを指定してリッスン
    loop{
        let (stream, addr) = listener.accept().await.unwrap(); //Conncetionの要求が来るまで待機, 接続が確立するとTcpStreamとクライアントのアドレスを取得
        println!("Connection from: {}", addr);
        tokio::spawn(async move {
            handle_connection(stream).await; // 新しい非同期タスクを生成して接続を処理. これにより複数の接続を同時に処理可能. streamはmoveクロージャに移動される
        });
    }
}


async fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream); 

    let mut req = vec![];
    let mut lines = buf_reader.lines();

    while let Some(line) = lines.next_line().await.unwrap(){
        if line.is_empty() {
            break;
        }
        req.push(line);
    }

    let res = format!("HTTP/1.1 200 OK\r\n\r\n{:#?}", req);
    stream.write_all(res.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
