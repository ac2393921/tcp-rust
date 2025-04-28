use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

// プログラムのエントリーポイント
fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数を取得します。
    // env::args() はプログラムのパスを含むイテレータを返すため、collect() でベクタに変換します。
    let args: Vec<String> = env::args().collect();
    // 最初の引数（プログラム名の次）をリッスンするアドレスとして取得します。
    // 例: cargo run 127.0.0.1:8080
    let addr = &args[1];
    // echo_server関数を呼び出してサーバーを開始します。
    echo_server(addr)?;
    // 成功した場合はOkを返します。
    Ok(())
}

// 指定されたアドレスでTCPエコーサーバーを開始する関数
fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    // 指定されたアドレスとポートでTCPリスナーをバインド（接続待機状態に）します。
    let listener = TcpListener::bind(address)?;
    println!("Listening on {}", address); // サーバーがリッスンを開始したことを示すメッセージ

    // 無限ループでクライアントからの接続を待ち受けます。
    loop {
        // 新しいクライアントからの接続を受け付けます。
        // accept() は接続が確立されるまでブロック（待機）し、
        // 成功するとクライアントとの通信用ストリーム (stream) とクライアントのアドレスをタプルで返します。
        let (mut stream, addr) = listener.accept()?;
        println!("Accepted connection from {}", addr); // 接続を受け付けたクライアントのアドレスを表示

        // 新しい接続ごとにスレッドを生成して、並行処理を行います。
        // move キーワードは、クロージャ（無名関数）が stream 変数の所有権を取得することを示します。
        thread::spawn(move || {
            // データを読み込むためのバッファを初期化します。サイズは1024バイトです。
            let mut buffer = [0u8; 1024];
            // クライアントからのデータを継続的に読み込むためのループ
            loop {
                // stream からデータを読み込み、バッファに格納します。
                // read() は読み込んだバイト数を返します。
                // エラーが発生した場合や接続が閉じた場合は unwrap() がパニックを引き起こします（エラーハンドリングは簡略化）。
                let nbytes = stream.read(&mut buffer).unwrap();

                // 読み込んだバイト数が0の場合、クライアントが接続を閉じたことを意味します。
                if nbytes == 0 {
                    println!("Connection closed by {}", addr); // 接続が閉じたことを示すメッセージ
                                                               // ループを抜けてスレッドを終了します。
                    return;
                }

                // 読み込んだデータをUTF-8文字列に変換して標準出力に表示します。
                // &[..nbytes] はバッファの読み込んだ部分だけをスライスとして取得します。
                // ここでもエラーハンドリングは簡略化されています。
                print!(
                    "Received data: {}",
                    str::from_utf8(&buffer[..nbytes]).unwrap()
                );

                // 受け取ったデータをそのままクライアントに書き戻します（エコー）。
                // write_all() は指定されたデータをすべて書き込むことを保証します。
                // ここでもエラーハンドリングは簡略化されています。
                stream.write_all(&buffer[..nbytes]).unwrap();
            }
        });
    }
}
