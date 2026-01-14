#!/bin/bash

# パスを引数で指定して、running_env/src配下にコピーするスクリプト

if [ $# -ne 1 ]; then
    echo "使用方法: $0 <sources配下のパス>"
    echo "例: $0 ../sources/ErrorHandling"
    exit 1
fi

# スクリプトのディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

SOURCE_PATH="$1"
DEST_DIR="$SCRIPT_DIR/running_env/src"

# ソースパスが存在するかチェック
if [ ! -e "$SOURCE_PATH" ]; then
    echo "エラー: $SOURCE_PATH が見つかりません"
    exit 1
fi

# running_env/src ディレクトリが存在するかチェック
if [ ! -d "$DEST_DIR" ]; then
    echo "エラー: $DEST_DIR ディレクトリが見つかりません"
    exit 1
fi

# コピーを実行
if [ -d "$SOURCE_PATH" ]; then
    # ソースコードファイル(.rs)だけを検索してコピー
    find "$SOURCE_PATH" -name "*.rs" | while read file; do
        cp "$file" "$DEST_DIR/"
        echo "✓ $(basename "$file") をコピーしました"
    done
else
    echo "エラー: $SOURCE_PATH はディレクトリではありません"
    exit 1
fi
