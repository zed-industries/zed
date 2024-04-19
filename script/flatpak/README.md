# Installing Locally

```
python flatpak-cargo-generator.py
flatpak-builder --user --install --force-clean build dev.zed.Zed.json
```

Note: Don't try changing the package manifest to build in debug mode, rust-embed will not like it and crash.