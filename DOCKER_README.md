# Docker環境での使用方法

## セットアップ

### 1. Dockerイメージをビルド
```bash
docker-compose build
```

### 2. コンテナを起動（対話モード）
```bash
docker-compose run --rm rust-env bash
```

### 3. コンテナ内でプログラムを実行
```bash
# ErrorHandlingを実行
cd running_env
./copy_to_running_env.sh ../sources/ErrorHandling
cargo run

# InstantWebServerを実行
./copy_to_running_env.sh ../sources/InstantWebServer
cargo run
```

## Windows / Mac での実行

### Windows (Docker Desktop)
1. [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop) をインストール
2. WSL2バックエンドを使用（推奨）
3. PowerShellまたはコマンドプロンプトで上記コマンドを実行

### Mac (Docker Desktop)
1. [Docker Desktop for Mac](https://www.docker.com/products/docker-desktop) をインストール
2. インストール完了後、ターミナルで上記コマンドを実行

## よく使うコマンド

### ビルド
```bash
docker-compose build
```

### 開発用に起動（ボリュームマウント）
```bash
docker-compose run --rm rust-env bash
```

### コンテナ内でcargo実行
```bash
docker-compose run --rm rust-env cargo run
```

### バックグラウンドで実行
```bash
docker-compose up -d
```

### ログを確認
```bash
docker-compose logs -f
```

### コンテナを停止
```bash
docker-compose down
```

## ファイル構成

```
Rust_Learning/
├── Dockerfile              # Rustビルド環境定義
├── docker-compose.yml      # Docker Compose設定
├── DOCKER_README.md        # このファイル
├── running_env/            # 実行環境
│   ├── Cargo.toml
│   └── src/
└── sources/                # ソースコード
    ├── ErrorHandling/
    ├── InstantWebServer/
    └── ...
```
