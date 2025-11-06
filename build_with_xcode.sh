#!/bin/bash
# Xcode'u environment variable ile kullanarak build

cd /Users/dogan/Desktop/zed

echo "🔧 Xcode environment variable ile aktif ediliyor..."
export DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer

echo "✅ DEVELOPER_DIR ayarlandı: $DEVELOPER_DIR"

# Metal tool'unu kontrol et
echo ""
echo "🔍 Metal tool kontrolü:"
xcrun --find metal 2>&1

echo ""
echo "🚀 Build başlatılıyor..."
source ~/.cargo/env
DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer cargo build

