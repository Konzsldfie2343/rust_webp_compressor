#!/bin/bash

# インストール先のパス
APP_PATH="/Applications/WebP Compressor.app"

# 検疫属性を再帰的に削除
if [ -d "$APP_PATH" ]; then
    /usr/bin/xattr -r -d com.apple.quarantine "$APP_PATH"
    echo "検疫属性を削除しました: $APP_PATH"
else
    echo "アプリケーションが見つかりません: $APP_PATH"
    exit 1
fi

exit 0