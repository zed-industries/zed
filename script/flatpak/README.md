# Installing Locally

To build & install the flatpak package locally, first install the necessary dependencies (replacing `x86_64` with your computers architecture):

```
flatpak install org.freedesktop.Platform/x86_64/23.08
flatpak install org.freedesktop.Sdk/x86_64/23.08
flatpak install org.freedesktop.Sdk.Extension.rust-stable/x86_64/23.08
flatpak install org.freedesktop.Sdk.Extension.golang/x86_64/23.08
```

Then, to build the actual package, generate the sources list and run flatpak builder, which will build and then install the package on your computer as a user installation.

```
python flatpak-cargo-generator.py
flatpak-builder --user --install --force-clean build dev.zed.Zed.json
```

> [!NOTE]
> This builds the Flatpak in release mode. Changing the manifest to build in debug mode will make the package crash at runtime because [rust-embed](https://github.com/pyrossh/rust-embed) only embeds files when built in release mode.

Lastly, to run the installed package, either use your desktop environments menu to find the application or run `flatpak run dev.zed.Zed`, which will start the application.