# RustEchoServerClient

Rustで簡易的なエコーサーバーとクライアントを実装。

## 特徴

- Rustで実装されたTCPエコーサーバーとクライアント
- サーバーは接続を受け付け、受信したメッセージをそのままクライアントに返します
- クライアントはメッセージを送信し、サーバーからの応答を表示

## 使用方法

1. リポジトリをクローンします: `git clone https://github.com/aida0710/RustEchoServerClient.git`
2. ディレクトリに移動します: `cd RustEchoServerClient`
3. サーバーとクライアントを起動します: `cargo run`

## ファイル構成

- `src/main.rs`: サーバーとクライアントのスレッドを起動するメイン関数
- `src/server.rs`: エコーサーバーを実装
- `src/client.rs`: エコーサーバーにメッセージを送信するクライアントを実装
