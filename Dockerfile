FROM rust:latest

WORKDIR /app

# ソースコードをコピー
COPY running_env/Cargo.toml ./Cargo.toml
COPY running_env/src ./src
COPY sources ./sources

# 依存関係をビルド
RUN cargo build --release

# ポート8080を公開
EXPOSE 8080

# デフォルトコマンド
CMD ["/bin/bash"]
