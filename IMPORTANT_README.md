# ⚠️ ÖNEMLİ: Xcode Aktif Etme Gerekiyor

## Durum
- ✅ Fork kurulumu tamamlandı
- ✅ Remote'lar ayarlandı  
- ✅ Branch oluşturuldu
- ✅ Build başlatıldı
- ⚠️ **Xcode aktif edilmesi gerekiyor** (sudo şifre gerektirir)

## Hemen Yapmanız Gereken

Terminal'de şu komutu çalıştırın:

```bash
cd /Users/dogan/Desktop/zed
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
sudo xcodebuild -license accept
```

**VEYA** hazırladığım script'i çalıştırın:

```bash
cd /Users/dogan/Desktop/zed
./RUN_THIS_NOW.sh
```

## Neden Gerekli?

Build sırasında Metal shader'ları compile edilirken Xcode'un aktif olması gerekiyor. Şu anda sistem Command Line Tools kullanıyor, bu yüzden Metal tool'u bulunamıyor.

## Xcode Aktif Edildikten Sonra

1. Build'in durumunu kontrol edin:
   ```bash
   cd /Users/dogan/Desktop/zed
   source ~/.cargo/env
   cargo build
   ```

2. Build tamamlandığında Zed'i çalıştırın:
   ```bash
   cargo run
   ```

## Build Durumu

Build şu anda devam ediyor, ancak Metal hatası nedeniyle durma ihtimali var. Xcode'u aktif ettikten sonra build otomatik olarak devam edecek veya tekrar başlatmanız gerekebilir.

## Sorun Giderme

Eğer hala Metal hatası alırsanız:

1. Xcode'u bir kez açın:
   ```bash
   open /Applications/Xcode.app
   ```
   Lisans sözleşmesini kabul edin.

2. Tekrar kontrol edin:
   ```bash
   xcode-select -p
   # Şunu görmelisiniz: /Applications/Xcode.app/Contents/Developer
   ```

