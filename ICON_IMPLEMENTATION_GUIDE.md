# NeuroNexus IDE Icon Implementation Guide

**Project:** NeuroNexus IDE  
**Developer:** cloudraLabs  
**Date:** November 5, 2025

---

## 🎨 Brand Identity

**Icon Design:** Neural network brain with vibrant gradient (cyan, orange, magenta)  
**Style:** Modern, tech-forward, neuroscience-inspired  
**Color Scheme:**
- Primary: Cyan/Blue (#00D9FF)
- Secondary: Orange (#FF8C00)
- Accent: Magenta/Red (#FF0066)
- Background: Dark navy/black

---

## 📋 Required Icon Files

The NeuroNexus IDE rebranding requires replacing icon files in multiple formats and resolutions across different platforms.

### macOS Icons (PNG Format)

**Location:** `crates/zed/resources/`

**Standard Release:**
- `app-icon.png` - 512x512px (1x resolution)
- `app-icon@2x.png` - 1024x1024px (2x retina resolution)

**Development Build:**
- `app-icon-dev.png` - 512x512px
- `app-icon-dev@2x.png` - 1024x1024px

**Nightly Build:**
- `app-icon-nightly.png` - 512x512px
- `app-icon-nightly@2x.png` - 1024x1024px

**Preview Build:**
- `app-icon-preview.png` - 512x512px
- `app-icon-preview@2x.png` - 1024x1024px

### Windows Icons (ICO Format)

**Location:** `crates/zed/resources/windows/`

**Multi-resolution ICO files:**
- `app-icon.ico` - Contains: 16x16, 32x32, 48x48, 256x256
- `app-icon-dev.ico` - Contains: 16x16, 32x32, 48x48, 256x256
- `app-icon-nightly.ico` - Contains: 16x16, 32x32, 48x48, 256x256
- `app-icon-preview.ico` - Contains: 16x16, 32x32, 48x48, 256x256

### Linux Icons

**Location:** `crates/zed/resources/`

Linux uses the same PNG files as macOS. Additional formats may be needed for:
- Desktop entries (`.desktop` files)
- Flatpak/Snap packages
- AppImage bundles

---

## 🛠️ Step-by-Step Implementation

### Step 1: Prepare the Source Icon

You have the brain icon image. Save it as a high-resolution PNG:

```bash
# Save your brain icon as:
neuronexus-icon-source.png (recommended: 2048x2048 or higher)
```

**Requirements:**
- ✅ Transparent background (PNG with alpha channel)
- ✅ Square aspect ratio (1:1)
- ✅ High resolution (at least 1024x1024, preferably 2048x2048)
- ✅ Clear at small sizes (icon should be recognizable at 16x16)

### Step 2: Generate PNG Variants

Use ImageMagick or a similar tool to create all required sizes:

```bash
# Install ImageMagick (if not already installed)
# macOS:
brew install imagemagick

# Linux:
sudo apt-get install imagemagick

# Windows:
# Download from https://imagemagick.org/

# Generate standard release icons
convert neuronexus-icon-source.png -resize 512x512 crates/zed/resources/app-icon.png
convert neuronexus-icon-source.png -resize 1024x1024 crates/zed/resources/app-icon@2x.png

# Generate dev build icons (optional: add a badge/overlay)
convert neuronexus-icon-source.png -resize 512x512 crates/zed/resources/app-icon-dev.png
convert neuronexus-icon-source.png -resize 1024x1024 crates/zed/resources/app-icon-dev@2x.png

# Generate nightly build icons (optional: different color overlay)
convert neuronexus-icon-source.png -resize 512x512 crates/zed/resources/app-icon-nightly.png
convert neuronexus-icon-source.png -resize 1024x1024 crates/zed/resources/app-icon-nightly@2x.png

# Generate preview build icons
convert neuronexus-icon-source.png -resize 512x512 crates/zed/resources/app-icon-preview.png
convert neuronexus-icon-source.png -resize 1024x1024 crates/zed/resources/app-icon-preview@2x.png
```

### Step 3: Generate Windows ICO Files

Windows ICO files contain multiple resolutions in a single file:

```bash
# Generate standard ICO (contains 16, 32, 48, 256)
convert neuronexus-icon-source.png \
  -resize 256x256 \
  \( -clone 0 -resize 16x16 \) \
  \( -clone 0 -resize 32x32 \) \
  \( -clone 0 -resize 48x48 \) \
  -delete 0 -colors 256 \
  crates/zed/resources/windows/app-icon.ico

# Repeat for other variants
convert neuronexus-icon-source.png \
  -resize 256x256 \
  \( -clone 0 -resize 16x16 \) \
  \( -clone 0 -resize 32x32 \) \
  \( -clone 0 -resize 48x48 \) \
  -delete 0 -colors 256 \
  crates/zed/resources/windows/app-icon-dev.ico

convert neuronexus-icon-source.png \
  -resize 256x256 \
  \( -clone 0 -resize 16x16 \) \
  \( -clone 0 -resize 32x32 \) \
  \( -clone 0 -resize 48x48 \) \
  -delete 0 -colors 256 \
  crates/zed/resources/windows/app-icon-nightly.ico

convert neuronexus-icon-source.png \
  -resize 256x256 \
  \( -clone 0 -resize 16x16 \) \
  \( -clone 0 -resize 32x32 \) \
  \( -clone 0 -resize 48x48 \) \
  -delete 0 -colors 256 \
  crates/zed/resources/windows/app-icon-preview.ico
```

### Step 4: Optional - Create macOS ICNS

For macOS app bundles, you may want to create an `.icns` file:

```bash
# Create iconset directory structure
mkdir -p neuronexus.iconset

# Generate all required sizes
sips -z 16 16     neuronexus-icon-source.png --out neuronexus.iconset/icon_16x16.png
sips -z 32 32     neuronexus-icon-source.png --out neuronexus.iconset/icon_16x16@2x.png
sips -z 32 32     neuronexus-icon-source.png --out neuronexus.iconset/icon_32x32.png
sips -z 64 64     neuronexus-icon-source.png --out neuronexus.iconset/icon_32x32@2x.png
sips -z 128 128   neuronexus-icon-source.png --out neuronexus.iconset/icon_128x128.png
sips -z 256 256   neuronexus-icon-source.png --out neuronexus.iconset/icon_128x128@2x.png
sips -z 256 256   neuronexus-icon-source.png --out neuronexus.iconset/icon_256x256.png
sips -z 512 512   neuronexus-icon-source.png --out neuronexus.iconset/icon_256x256@2x.png
sips -z 512 512   neuronexus-icon-source.png --out neuronexus.iconset/icon_512x512.png
sips -z 1024 1024 neuronexus-icon-source.png --out neuronexus.iconset/icon_512x512@2x.png

# Convert to ICNS
iconutil -c icns neuronexus.iconset -o neuronexus.icns

# Clean up
rm -rf neuronexus.iconset
```

### Step 5: Verify Icon Files

Check that all files were created correctly:

```bash
# List all icon files
ls -lh crates/zed/resources/app-icon*.png
ls -lh crates/zed/resources/windows/app-icon*.ico

# Verify PNG dimensions
file crates/zed/resources/app-icon.png
file crates/zed/resources/app-icon@2x.png

# Verify ICO contains multiple sizes
identify crates/zed/resources/windows/app-icon.ico
```

---

## 🎨 Design Variations for Build Types

### Standard Release
- Clean, professional brain icon
- Full color gradient
- No overlays

### Development Build (`-dev`)
**Suggestion:** Add a small "DEV" badge or change the gradient to include more yellow/orange

```bash
# Example: Add text overlay
convert app-icon.png \
  -gravity SouthEast \
  -pointsize 60 \
  -fill '#FFD700' \
  -annotate +10+10 'DEV' \
  app-icon-dev.png
```

### Nightly Build (`-nightly`)
**Suggestion:** Darker theme or purple/violet tint to differentiate

```bash
# Example: Add purple tint
convert app-icon.png \
  -modulate 100,150,100 \
  -colorize 10,0,20 \
  app-icon-nightly.png
```

### Preview Build (`-preview`)
**Suggestion:** Add "PREVIEW" badge or slightly transparent overlay

---

## 📐 Technical Specifications

### PNG Requirements
- **Format:** PNG with alpha transparency
- **Color Space:** sRGB
- **Bit Depth:** 24-bit RGB + 8-bit alpha (32-bit total)
- **Compression:** PNG standard compression
- **Metadata:** Include copyright/author info

### ICO Requirements
- **Embedded Sizes:** 16x16, 32x32, 48x48, 256x256
- **Color Depth:** 32-bit (24-bit RGB + 8-bit alpha)
- **Compression:** Uncompressed or PNG-compressed
- **Compatibility:** Windows 7+

### File Sizes (Approximate)
- `app-icon.png` (512x512): ~150-200 KB
- `app-icon@2x.png` (1024x1024): ~400-600 KB
- `app-icon.ico`: ~150-250 KB

---

## 🔧 Configuration Files

The icon paths are configured in:

### 1. Windows Build Script
**File:** `crates/zed/build.rs`

```rust
let icon = match release_channel {
    "stable" => "resources/windows/app-icon.ico",
    "preview" => "resources/windows/app-icon-preview.ico",
    "nightly" => "resources/windows/app-icon-nightly.ico",
    "dev" => "resources/windows/app-icon-dev.ico",
    _ => "resources/windows/app-icon-dev.ico",
};
```

**No changes needed** - paths remain the same, just replace the icon files.

### 2. macOS Info.plist
**File:** `crates/zed/resources/info/...`

macOS automatically uses PNG files from the resources directory. The build system handles this.

### 3. Linux Desktop Entry
**File:** `crates/zed/resources/zed.desktop.in`

```ini
Icon=zed
```

The icon is referenced by name. Ensure the PNG files are installed to the correct system locations.

---

## 🚀 Build and Test

### 1. Build the Application

```bash
cd /Users/cleitonmouraloura/Documents/NeuroNexusIDE

# Debug build (faster, for testing)
cargo build

# Release build (optimized)
cargo build --release
```

### 2. Test Icon Display

**macOS:**
```bash
# Run the app
./target/debug/zed
# or
./target/release/zed

# Check Dock icon
# Check app bundle icon (Finder)
# Check About dialog
```

**Windows:**
```bash
# Run the app
.\target\debug\zed.exe
# or
.\target\release\zed.exe

# Check taskbar icon
# Check .exe file icon (Explorer)
# Check window title bar icon
```

**Linux:**
```bash
# Run the app
./target/debug/zed
# or
./target/release/zed

# Check window manager icon
# Check application menu icon
```

### 3. Verify Icon Quality

Check icon appearance at different sizes:
- **16x16** - System tray, taskbar (small)
- **32x32** - Window title bar
- **48x48** - Alt+Tab switcher
- **256x256** - Application folder, About dialog
- **512x512** - macOS Dock, high-DPI displays

---

## 🎯 Quality Checklist

Before finalizing:

- [ ] All PNG files created (8 files)
- [ ] All ICO files created (4 files)
- [ ] Icons display correctly at 16x16 (smallest size)
- [ ] Icons display correctly at 512x512+ (largest size)
- [ ] Transparent background maintained
- [ ] Colors are vibrant and accurate
- [ ] No compression artifacts
- [ ] Build types differentiated (dev/nightly/preview)
- [ ] Icons tested on all target platforms
- [ ] File sizes are reasonable (<1MB each)
- [ ] Copyright/attribution metadata included

---

## 🌐 Platform-Specific Notes

### macOS
- Retina displays require @2x versions
- PNG format is standard
- ICNS optional but recommended for app bundles
- Icon caching may require logout/login to see changes

### Windows
- ICO format required for .exe files
- Multiple resolutions in single file
- High-DPI support via 256x256 embedded size
- May need to rebuild icon cache: `ie4uinit.exe -show`

### Linux
- PNG format standard
- Icon themes may override
- Desktop files reference by name
- Install to `/usr/share/icons/hicolor/*/apps/`

---

## 🔄 Update Process

When updating icons in the future:

1. **Replace source icon:** Update `neuronexus-icon-source.png`
2. **Regenerate all sizes:** Run the conversion scripts
3. **Increment version:** Update extension/app version numbers
4. **Rebuild:** `cargo clean && cargo build --release`
5. **Test:** Verify on all platforms
6. **Commit:** Git commit with clear message
7. **Tag release:** Git tag for version tracking

---

## 📦 Distribution

### App Bundles
Icons are embedded in:
- **macOS:** `.app/Contents/Resources/`
- **Windows:** `.exe` resource section
- **Linux:** Copied to system icons directory

### Installers
- **macOS DMG:** Uses app bundle icon
- **Windows Installer:** Configured in `zed.iss`
- **Linux packages:** Defined in package metadata

---

## 🆘 Troubleshooting

### Icon not updating after build
```bash
# Clear build cache
cargo clean

# macOS: Reset icon cache
sudo rm -rf /Library/Caches/com.apple.iconservices.store
killall Dock

# Windows: Rebuild icon cache
ie4uinit.exe -show

# Linux: Update icon cache
gtk-update-icon-cache -f -t /usr/share/icons/hicolor
```

### Icon looks blurry
- Check source resolution (should be 2048x2048 or higher)
- Verify PNG has no JPEG artifacts
- Ensure @2x versions are exactly 2x the 1x size
- Use PNG optimization tools without quality loss

### Icon has white/black background
- Source must have transparent alpha channel
- Check PNG bit depth (should be 32-bit with alpha)
- Verify with: `identify -verbose icon.png | grep Alpha`

### Colors look wrong
- Verify sRGB color space
- Check gamma correction
- Test on different displays
- Use color management tools

---

## 📚 Resources

### Tools
- **ImageMagick:** https://imagemagick.org/
- **GIMP:** https://www.gimp.org/ (GUI icon editor)
- **Inkscape:** https://inkscape.org/ (vector graphics)
- **Icon Slate:** https://www.kodlian.com/apps/icon-slate (macOS)
- **IcoFX:** https://icofx.ro/ (Windows ICO editor)

### Icon Guidelines
- **macOS Human Interface Guidelines:** https://developer.apple.com/design/human-interface-guidelines/app-icons
- **Windows App Icon Guidelines:** https://docs.microsoft.com/en-us/windows/apps/design/style/iconography
- **Freedesktop Icon Theme Spec:** https://specifications.freedesktop.org/icon-theme-spec/

---

## ✅ Summary

**Quick Reference:**
```bash
# 1. Save source icon (2048x2048 PNG with transparency)
# 2. Generate PNG variants:
convert source.png -resize 512x512 app-icon.png
convert source.png -resize 1024x1024 app-icon@2x.png

# 3. Generate ICO files:
convert source.png [multi-size-options] app-icon.ico

# 4. Copy to correct locations
# 5. Build and test
cargo build --release

# 6. Verify icons on all platforms
```

---

**Developer:** cloudraLabs  
**Project:** NeuroNexus IDE  
**Last Updated:** November 5, 2025
