# Zed Fork Kurulum Rehberi

Bu rehber, Zed'i kendi kullanımınız için fork etmenize yardımcı olur.

## Mevcut Durum

✅ Zed repository'si clone edilmiş: `/Users/dogan/Desktop/zed`
✅ Rust toolchain kurulu (1.90)
✅ Cmake kurulu
✅ Build başlatıldı

## Adım 1: GitHub'da Fork Oluşturma (Opsiyonel)

Eğer değişikliklerinizi GitHub'da saklamak istiyorsanız:

1. GitHub'da [Zed repository](https://github.com/zed-industries/zed)'sine gidin
2. Sağ üstteki "Fork" butonuna tıklayın
3. Fork'u kendi hesabınıza oluşturun

## Adım 2: Remote'u Kendi Fork'unuza Ayarlama

```bash
cd /Users/dogan/Desktop/zed

# Mevcut remote'u kontrol edin
git remote -v

# Kendi fork'unuzun URL'sini ekleyin (YOUR_USERNAME'i değiştirin)
git remote add fork https://github.com/YOUR_USERNAME/zed.git

# Veya mevcut origin'i değiştirin
git remote set-url origin https://github.com/YOUR_USERNAME/zed.git
```

## Adım 3: Build ve Çalıştırma

### Debug Build (Daha Hızlı)
```bash
cd /Users/dogan/Desktop/zed
source "$HOME/.cargo/env"
cargo build
```

### Release Build (Daha Yavaş ama Optimize)
```bash
cargo build --release
```

### Çalıştırma
```bash
# Debug build
cargo run

# Release build
cargo run --release
```

## Adım 4: Kendi Branch'inizi Oluşturma

Değişiklikleriniz için yeni bir branch oluşturun:

```bash
git checkout -b my-custom-features
```

## Adım 5: Modifikasyonlar Yapma

Artık istediğiniz değişiklikleri yapabilirsiniz:

- Ollama entegrasyonunu özelleştirme
- Yeni özellikler ekleme
- UI değişiklikleri
- vs.

## Önemli Notlar

⚠️ **Lisans**: Zed AGPL-3.0 lisansı altında. Kendi kullanımınız için sorun yok, ama dağıtırsanız kaynak kodunu açmanız gerekir.

✅ **Kendi Kullanımı**: Sadece kendi bilgisayarınızda kullanıyorsanız hiçbir sorun yok.

## Build Süresi

İlk build genellikle 30-60 dakika sürebilir. Sonraki build'ler çok daha hızlı olacaktır.

## Sorun Giderme

### Xcode Hatası
```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

### Cmake Hatası
```bash
brew install cmake
```

### Rust Versiyonu
```bash
rustup override set 1.90
```

## Sonraki Adımlar

1. Build'in tamamlanmasını bekleyin
2. `cargo run` ile Zed'i çalıştırın
3. İstediğiniz modifikasyonları yapın
4. Değişikliklerinizi commit edin

