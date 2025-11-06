#!/bin/bash
# Final build script - Xcode environment variable ile

cd /Users/dogan/Desktop/zed

echo "🔧 Xcode environment variable ayarlanıyor..."
export DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer

echo "📦 Metal Toolchain kontrol ediliyor..."
xcodebuild -downloadComponent MetalToolchain 2>&1 | head -10

echo ""
echo "🚀 Build başlatılıyor..."
source ~/.cargo/env

# Build'i başlat (arka planda çalışabilir)
DEVELOPER_DIR=/Applications/Xcode.app/Contents/Developer cargo build 2>&1 | tee build.log

echo ""
echo "✅ Build tamamlandı! (veya hata var, build.log dosyasına bakın)"

