#!/bin/bash
# Fork işlemi tamamlandıktan sonra çalıştırılacak script

cd /Users/dogan/Desktop/zed

echo "🔧 Fork sonrası kurulum başlatılıyor..."

# GitHub kullanıcı adınızı buraya yazın (görselde senoldogann görünüyor)
GITHUB_USER="senoldogann"

# Mevcut origin'i upstream olarak ekle
echo "📦 Orijinal repository'yi upstream olarak ekleniyor..."
git remote add upstream https://github.com/zed-industries/zed.git 2>/dev/null || echo "Upstream zaten mevcut"

# Origin'i kendi fork'unuza ayarla
echo "🔗 Remote'u kendi fork'unuza ayarlanıyor..."
git remote set-url origin https://github.com/${GITHUB_USER}/zed.git

# Remote'ları kontrol et
echo ""
echo "✅ Remote'lar ayarlandı:"
git remote -v

# Kendi branch'inizi oluştur
echo ""
echo "🌿 Kendi branch'iniz oluşturuluyor..."
git checkout -b my-custom-features 2>/dev/null || git checkout my-custom-features

echo ""
echo "✅ Kurulum tamamlandı!"
echo ""
echo "📝 Sonraki adımlar:"
echo "1. Değişikliklerinizi yapın"
echo "2. git add ."
echo "3. git commit -m 'Your changes'"
echo "4. git push origin my-custom-features"
echo ""
echo "🚀 Zed'i çalıştırmak için:"
echo "   source ~/.cargo/env && cargo run"

