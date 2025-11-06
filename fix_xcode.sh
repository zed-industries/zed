#!/bin/bash
# Xcode'u aktif hale getirmek için script

echo "🔧 Xcode aktif hale getiriliyor..."

# Xcode'u aktif et (sudo gerekli)
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer

# Xcode lisansını kabul et
sudo xcodebuild -license accept

# Kontrol et
echo ""
echo "✅ Xcode aktif edildi:"
xcode-select -p

echo ""
echo "🔍 Metal tool kontrolü:"
xcrun --find metal 2>&1 || echo "Metal bulunamadı, Xcode'u açıp lisansı kabul etmeniz gerekebilir"

