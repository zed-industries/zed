# Zed

<p align="center">
  <a href="README.md">English</a> ·
  <strong>中文</strong>
</p>

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

欢迎使用 Zed，这是一款高性能、支持多人协作的代码编辑器，由 [Atom](https://github.com/atom/atom) 和 [Tree-sitter](https://github.com/tree-sitter/tree-sitter) 的主创团队倾力打造。

---

### 安装

在 macOS、Linux 和 Windows 上，你可以[直接下载 Zed](https://zed.dev/download)，或者通过你的本地包管理器安装 Zed（[macOS](https://zed.dev/docs/installation#macos)/[Linux](https://zed.dev/docs/linux#installing-via-a-package-manager)/[Windows](https://zed.dev/docs/windows#package-managers)）。

其他平台暂未支持：

- Web（[跟踪 issue](https://github.com/zed-industries/zed/issues/5396)）

### 开发 Zed

- [为 macOS 构建 Zed](./docs/src/development/macos.md)
- [为 Linux 构建 Zed](./docs/src/development/linux.md)
- [为 Windows 构建 Zed](./docs/src/development/windows.md)

### 参与贡献

请参阅 [CONTRIBUTING.md](./CONTRIBUTING.md) 了解你可以如何为 Zed 做出贡献。

此外……我们正在招聘！请查看我们的[招聘](https://zed.dev/jobs)页面了解空缺职位。

### 许可证

必须正确提供第三方依赖的许可证信息，CI 才能通过。

我们使用 [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) 来自动遵守开源许可证。如果 CI 失败，请检查以下内容：

- 是否为你创建的 crate 显示了 `no license specified` 错误？如果是，请在你的 crate 的 Cargo.toml 文件中的 `[package]` 下添加 `publish = false`。
- 是否为某个依赖显示了 `failed to satisfy license requirements` 错误？如果是，首先确定该项目使用的是什么许可证，以及该系统是否足以满足该许可证的要求。如果你不确定，请咨询律师。一旦你确认此系统是可以接受的，请将该许可证的 SPDX 标识符添加到 `script/licenses/zed-licenses.toml` 中的 `accepted` 数组中。
- `cargo-about` 是否无法找到某个依赖的许可证？如果是，请按照 [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration) 中的指定，在 `script/licenses/zed-licenses.toml` 的末尾添加一个 clarification 字段。

## 赞助

Zed 由 **Zed Industries, Inc.**（一家营利性公司）开发。

如果你想从财务上支持该项目，可以通过 GitHub Sponsors 进行赞助。
赞助金直接拨给 Zed Industries，作为公司的一般收入。
赞助并不附带任何特权或权利。