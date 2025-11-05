# 🎨 NeuroNexus Icon Generator

**Developer:** cloudraLabs  
**Project:** NeuroNexus IDE

---

## Quick Start

### 1. Install Dependencies

**macOS:**
```bash
brew install imagemagick optipng
```

**Ubuntu/Debian:**
```bash
sudo apt-get install imagemagick optipng
```

**Fedora/RHEL:**
```bash
sudo dnf install ImageMagick optipng
```

### 2. Run the Script

**Basic usage** (generates standard icons only):
```bash
./generate-icons.sh path/to/your-brain-icon.png
```

**Recommended** (generates all variants with badges):
```bash
./generate-icons.sh --variants path/to/your-brain-icon.png
```

---

## What It Does

The script automatically generates **12 icon files** in the correct formats and sizes:

### PNG Icons (8 files)
- ✅ `app-icon.png` (512x512)
- ✅ `app-icon@2x.png` (1024x1024 - Retina)
- ✅ `app-icon-dev.png` (512x512 with DEV badge)
- ✅ `app-icon-dev@2x.png` (1024x1024 with DEV badge)
- ✅ `app-icon-nightly.png` (512x512 with purple tint)
- ✅ `app-icon-nightly@2x.png` (1024x1024 with purple tint)
- ✅ `app-icon-preview.png` (512x512 with PREVIEW badge)
- ✅ `app-icon-preview@2x.png` (1024x1024 with PREVIEW badge)

### Windows ICO Icons (4 files)
- ✅ `app-icon.ico` (multi-resolution: 16, 32, 48, 64, 96, 128, 256)
- ✅ `app-icon-dev.ico`
- ✅ `app-icon-nightly.ico`
- ✅ `app-icon-preview.ico`

---

## Usage

### Command Syntax

```bash
./generate-icons.sh [OPTIONS] <input-image>
```

### Options

| Option | Description |
|--------|-------------|
| `-h, --help` | Show help message |
| `-o, --output DIR` | Specify output directory (default: current dir) |
| `-v, --variants` | Generate variant icons with badges/tints |
| `-n, --no-optimize` | Skip PNG optimization (faster) |
| `-q, --quiet` | Quiet mode (less output) |

### Examples

**Generate all icons with variants:**
```bash
./generate-icons.sh --variants neuronexus-brain-icon.png
```

**Custom output directory:**
```bash
./generate-icons.sh --output ~/Desktop neuronexus-brain-icon.png
```

**Fast generation without optimization:**
```bash
./generate-icons.sh --variants --no-optimize icon.png
```

**From a different directory:**
```bash
cd /Users/cleitonmouraloura/Documents/NeuroNexusIDE
./generate-icons.sh --variants ~/Downloads/brain-icon.png
```

---

## Input Requirements

### Image Specifications

✅ **Format:** PNG (recommended), JPEG, or any ImageMagick-supported format  
✅ **Resolution:** Minimum 512x512, recommended 1024x1024 or higher  
✅ **Ideal:** 2048x2048 for best quality  
✅ **Transparency:** Alpha channel recommended (transparent background)  
✅ **Aspect Ratio:** Square (1:1) - will be auto-cropped if not square  
✅ **Color Space:** sRGB recommended

### Quality Tips

- **Higher resolution = better quality** at all sizes
- **Transparent background** works best across all platforms
- **Bold, simple designs** work better at small sizes (16x16)
- **Test at 16x16** to ensure it's recognizable when small

---

## Output Locations

Icons are generated in the standard Zed project structure:

```
NeuroNexusIDE/
├── crates/zed/resources/
│   ├── app-icon.png
│   ├── app-icon@2x.png
│   ├── app-icon-dev.png
│   ├── app-icon-dev@2x.png
│   ├── app-icon-nightly.png
│   ├── app-icon-nightly@2x.png
│   ├── app-icon-preview.png
│   ├── app-icon-preview@2x.png
│   └── windows/
│       ├── app-icon.ico
│       ├── app-icon-dev.ico
│       ├── app-icon-nightly.ico
│       └── app-icon-preview.ico
```

---

## Build Variants

### Standard Release
- Clean icon with no modifications
- Used for production builds

### Development (`-dev`)
- **Badge:** Yellow "DEV" text in bottom-right corner
- **Purpose:** Distinguish development builds

### Nightly (`-nightly`)
- **Effect:** Purple tint overlay
- **Purpose:** Identify nightly/experimental builds

### Preview (`-preview`)
- **Badge:** Orange "PREVIEW" text in top-right corner
- **Purpose:** Mark preview/beta releases

---

## What the Script Does

### 1. Dependency Check
- ✅ Verifies ImageMagick is installed
- ✅ Checks for optional optipng (for optimization)
- ✅ Reports version information

### 2. Input Validation
- ✅ Verifies image file exists
- ✅ Checks image format is valid
- ✅ Reports dimensions and color space
- ✅ Warns if resolution is low
- ✅ Checks for transparency

### 3. PNG Generation
- ✅ Resizes to exact dimensions
- ✅ Maintains aspect ratio
- ✅ Preserves transparency
- ✅ Creates 1x and 2x versions
- ✅ Adds variant badges/effects (if --variants)
- ✅ Optimizes file size (if optipng available)

### 4. ICO Generation
- ✅ Creates multi-resolution ICO files
- ✅ Embeds 7 sizes: 16, 32, 48, 64, 96, 128, 256
- ✅ Maintains transparency
- ✅ Windows-compatible format

### 5. Report Generation
- ✅ Lists all generated files
- ✅ Shows file sizes
- ✅ Provides next steps

---

## After Generation

### 1. Review Icons

Check the generated icons:

```bash
# View PNG icons
open crates/zed/resources/app-icon*.png

# Check file sizes
ls -lh crates/zed/resources/app-icon*.png
ls -lh crates/zed/resources/windows/app-icon*.ico
```

### 2. Build the Application

```bash
# Clean build (recommended)
cargo clean
cargo build --release

# Or quick debug build
cargo build
```

### 3. Test Icons

**macOS:**
- Run the app and check Dock icon
- Check Finder icon (app bundle)
- Check About dialog

**Windows:**
- Run the .exe and check taskbar icon
- Check Explorer icon (.exe file)
- Check window title bar

**Linux:**
- Check window manager icon
- Check application menu

### 4. Commit Changes

```bash
git add crates/zed/resources/app-icon*.png
git add crates/zed/resources/windows/app-icon*.ico
git commit -m "Update icons for NeuroNexus rebranding by cloudraLabs"
```

---

## Troubleshooting

### "ImageMagick not found"

**Solution:**
```bash
# macOS
brew install imagemagick

# Ubuntu/Debian
sudo apt-get install imagemagick

# Fedora/RHEL
sudo dnf install ImageMagick
```

### "Invalid image file"

**Causes:**
- Corrupt image file
- Unsupported format
- File doesn't exist

**Solution:**
- Verify file path is correct
- Try converting to PNG first: `convert input.jpg output.png`
- Check file isn't corrupted

### Icons look blurry

**Causes:**
- Source resolution too low
- JPEG compression artifacts

**Solution:**
- Use higher resolution source (2048x2048+)
- Use PNG format with no compression
- Ensure source has clean, sharp edges

### "Permission denied"

**Solution:**
```bash
chmod +x generate-icons.sh
```

### Icons not updating after build

**Solution:**
```bash
# Clear build cache
cargo clean

# macOS: Reset icon cache
sudo rm -rf /Library/Caches/com.apple.iconservices.store
killall Dock

# Linux: Update icon cache
gtk-update-icon-cache -f -t /usr/share/icons/hicolor
```

---

## Advanced Usage

### Custom Badge Colors

Edit the script to customize badge colors:

```bash
# Line ~290 (DEV badge)
-fill '#FFD700'    # Change to your preferred color

# Line ~330 (Nightly tint)
-fill '#8B00FF'    # Change to your preferred color

# Line ~370 (Preview badge)
-fill '#FF6B35'    # Change to your preferred color
```

### Different Badge Text

Change badge text:

```bash
# Line ~295 (DEV badge)
-annotate +20+20 'DEV'    # Change 'DEV' to your text

# Line ~375 (Preview badge)
-annotate +15+15 'PREVIEW'    # Change 'PREVIEW' to your text
```

### Skip Specific Variants

Comment out sections you don't need:

```bash
# To skip dev variant, comment out lines ~285-315
# To skip nightly variant, comment out lines ~325-355
# To skip preview variant, comment out lines ~365-395
```

---

## Performance

**Typical run time:** 10-30 seconds

**With optimization:** ~20-40 seconds  
**Without optimization:** ~5-15 seconds

Use `--no-optimize` for faster generation during testing.

---

## File Sizes

**Expected file sizes:**

| File | Approximate Size |
|------|-----------------|
| app-icon.png (512x512) | 150-250 KB |
| app-icon@2x.png (1024x1024) | 400-700 KB |
| app-icon.ico | 150-300 KB |

**Total:** ~2-4 MB for all 12 files

With optimization (`optipng`), files may be 20-40% smaller.

---

## Script Features

✅ **Fully automated** - One command generates everything  
✅ **Dependency checking** - Warns if tools missing  
✅ **Input validation** - Checks image quality  
✅ **Error handling** - Stops on errors  
✅ **Progress reporting** - Shows what's happening  
✅ **File size reporting** - Shows generated file sizes  
✅ **Colored output** - Easy to read  
✅ **Variant support** - Optional badges and tints  
✅ **Optimization** - Optional PNG compression  
✅ **Cross-platform** - Works on macOS, Linux, Windows (WSL)

---

## Technical Details

### PNG Generation
- Uses ImageMagick `convert` command
- Preserves alpha channel (transparency)
- Centers image if not square
- Resizes using high-quality filtering
- Optimizes with optipng (optional)

### ICO Generation
- Creates multi-resolution ICO files
- Embeds 7 sizes in one file
- Maintains 32-bit color depth
- Preserves transparency
- Windows Vista+ compatible

### Color Management
- Maintains original color space
- Preserves color profiles
- No color quantization
- No palette reduction

---

## Support

**Issues?** Check:
1. Dependencies installed correctly
2. Input image is valid
3. Sufficient disk space
4. Write permissions in output directory

**Still stuck?** Review:
- `ICON_IMPLEMENTATION_GUIDE.md` - Detailed manual process
- ImageMagick documentation - https://imagemagick.org/
- Script source code - Well-commented

---

## License

Part of NeuroNexus IDE  
Same license as Zed (Apache 2.0 / GPL)

---

**Developer:** cloudraLabs  
**Last Updated:** November 5, 2025  
**Version:** 1.0
