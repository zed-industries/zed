# msvc_spectre_libs
Provides an easy way to link with the [Spectre-mitigated](https://docs.microsoft.com/en-us/cpp/build/reference/qspectre) libraries for `msvc` targets.

To use, add this crate as a `dependency` in your `Cargo.toml`.

The Spectre-mitigated libs can be installed using Visual Studio Installer. For example:
`MSVC v143 - VS 2022 C++ x64/x86 Spectre-mitigated libs (Latest)`

## Error handling
If the Spectre-mitigated libs are not available, the build will issue a warning and continue.

Alternatively, to make the build panic instead, activate the `error` feature of this crate.

## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

## Trademarks

This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft 
trademarks or logos is subject to and must follow 
[Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general).
Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship.
Any use of third-party trademarks or logos are subject to those third-party's policies.