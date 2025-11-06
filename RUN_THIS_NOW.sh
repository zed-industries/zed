#!/bin/bash
# Bu script'i terminal'de çalıştırın (şifre isteyecek)

echo "🔧 Xcode aktif hale getiriliyor..."
echo "⚠️  Şifre isteyecek, lütfen girin..."

# Xcode'u aktif et
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer

# Xcode lisansını kabul et
sudo xcodebuild -license accept

echo ""
echo "✅ Xcode aktif edildi!"
echo ""
echo "Kontrol:"
xcode-select -p
echo ""
xcrun --find metal

