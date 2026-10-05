#!/bin/bash
# build.sh
set -e # 任何命令失败立即退出

TARGET_DIR="target/x86_64-pc-windows-gnu/debug"
LIB_NAME="forge.dll"
LIB_PATH="$TARGET_DIR/$LIB_NAME"
BACKUP_PATH="$TARGET_DIR/$LIB_NAME.backup"

# 1. 备份当前可用的库
if [ -f "$LIB_PATH" ]; then
    cp "$LIB_PATH" "$BACKUP_PATH"
fi

# 2. 尝试编译（输出到临时目录或直接编译）
if cargo dev; then
    echo "✅ 编译成功，库已更新"
else
    echo "❌ 编译失败，正在恢复备份..."
    if [ -f "$BACKUP_PATH" ]; then
        mv "$BACKUP_PATH" "$LIB_PATH"
        echo "✅ 已恢复上一个可用的库文件"
    else
        echo "⚠️ 没有找到备份文件，无法恢复"
    fi
    # exit 1
fi
