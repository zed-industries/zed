# Xcode Aktif Etme Rehberi

## Sorun
Xcode kurulu ama aktif değil. Sistem şu anda Command Line Tools kullanıyor, bu yüzden Metal tool'u bulunamıyor.

## Çözüm

### Adım 1: Xcode'u Aktif Et
Terminal'de şu komutu çalıştırın (şifre isteyecek):

```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

### Adım 2: Xcode Lisansını Kabul Et
```bash
sudo xcodebuild -license accept
```

### Adım 3: Xcode'u Açın (İlk Kez)
Eğer Xcode'u hiç açmadıysanız, bir kez açıp lisansı kabul etmeniz gerekebilir:

```bash
open /Applications/Xcode.app
```

Xcode açıldığında lisans sözleşmesini kabul edin.

### Adım 4: Kontrol Edin
```bash
# Xcode'un aktif olduğunu kontrol edin
xcode-select -p
# Şunu görmelisiniz: /Applications/Xcode.app/Contents/Developer

# Metal tool'unu kontrol edin
xcrun --find metal
```

### Adım 5: Build'i Tekrar Deneyin
```bash
cd /Users/dogan/Desktop/zed
source ~/.cargo/env
cargo build
```

## Alternatif: Script Kullanma

Hazırladığım script'i kullanabilirsiniz:

```bash
cd /Users/dogan/Desktop/zed
./fix_xcode.sh
```

## Notlar

- `sudo` komutu şifre isteyecektir
- Xcode'u ilk kez açıyorsanız, lisans sözleşmesini kabul etmeniz gerekir
- Metal tool'u Xcode'un içinde gelir, ayrı kurulum gerekmez

