These `.winmd` files provide the default metadata for the Windows API. This is used to
generate the `windows` and `windows-sys` crates. To view the metadata, use a tool
like [ILSpy](https://github.com/icsharpcode/ILSpy).

## `Windows.Win32.winmd`

- Source: <https://www.nuget.org/packages/Microsoft.Windows.SDK.Win32Metadata/>
- Version: `63.0.31`

## `Windows.Wdk.winmd`

- Source: <https://www.nuget.org/packages/Microsoft.Windows.WDK.Win32Metadata/>
- Version: `0.13.25`

## `Windows.winmd`

- Source: <https://www.nuget.org/packages/Microsoft.Windows.SDK.Contracts>
- Version: `10.0.26100.1742`

The `Windows.winmd` file was created by merging the .winmd files from the last nuget package as follows:

```sh
mdmerge -o out -i . -n 1
```

---

As with everything else in this repo, the `.winmd` files in this folder are licensed via MIT or Apache-2.0.
