# Bugs n fixes
These are some fixes to bugs found in the Windows version of Zed.

> [!NOTE]
> Feel free to create a PR to add your solutions :)

## `Crashes after a few minutes of use`
- **Issue**: Zed crashes after a few minutes of use on Windows on Laptop. [related Github issue](https://github.com/zed-industries/zed/issues/11192)
- **Solution**: For me, the issue was resolved by setting Zed to use the dedicated GPU instead of the integrated GPU. This can be done by changing the graphics settings in Windows 10/11 settings. [Settings -> System -> Display -> Graphics -> Add an app -> Select your Zed executable -> Options -> High performance](https://pureinfotech.com/set-gpu-app-windows-10/)

## `Language Servers do not start/work`
- **Issue**: The language servers dont start or dont work on the Windows build of Zed. [related Github issue](https://github.com/zed-industries/zed/issues/4628)
- **Solution**: Solution was to use a [PR](https://github.com/zed-industries/zed/pull/12036) created by [d1y](https://github.com/d1y), but is not approved by the Zed team yet, though it works for me. This PR is already merged to this repo.

## `Zed white window flash on startup`
- **Issue**: On Zed startup, a white window flashes for a ~400ms and then disappears, does not cause any hindering in development but is aesthetically annoying.
- **Solution**: Unknown. If you know the solution, feel free to create a PR to add it here.
