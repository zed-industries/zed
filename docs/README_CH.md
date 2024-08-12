<h4 align="center">
    <p>
        <a href="../README.md">English</b> |
        <a gref="./README_ES.md">Español</a> |
        <b>普通话</b>
    </p>
</h4>

# Zed

[![CI](https://github.com/zed-industries/zed/actions/workflows/ci.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/ci.yml)

欢迎使用 Zed，这是一款来自[Atom](https://github.com/atom/atom) 和 [Tree-sitter](https://github.com/tree-sitter/tree-sitter) 创建者的高性能多人代码编辑器

--------

### 设施


<a href="https://repology.org/project/zed-editor/versions">
    <img src="https://repology.org/badge/vertical-allrepos/zed-editor.svg?minversion=0.143.5" alt="Packaging status" align="right">
</a>

在 macOS 和 Linux 上，您可以[直接下载 Zed](https://zed.dev/download) 或[通过本地包管理器安装 Zed](https://zed.dev/docs/linux#installing-via -a -包管理器）。

Otras plataformas aún no están disponibles:

- Windows ([tracking issue](https://github.com/zed-industries/zed/issues/5394))
- Web ([tracking issue](https://github.com/zed-industries/zed/issues/5396))

### 开发 Zed

- [为 macOS 构建 Zed](./docs/src/development/macos.md)
- [为 Linux 构建 Zed](./docs/src/development/linux.md)
- [Zed Windows 版本] (./docs/src/development/windows.md)
- [运行本地协作] (./docs/src/development/local-collaboration.md)

### 贡献

请参阅 [CONTRIBUTING.md](../CONTRIBUTING.md) 了解为 Zed 做出贡献的方式。

另外...我们正在招聘！检查我们的[职位](https://zed.dev/jobs) 页面以获取空缺职位。

### 许可证

必须正确提供第三方依赖项的许可证信息才能通过 CI。

我们使用 [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) 自动遵守开源许可证。如果 CI 失败，请检查以下内容：

- 您创建的盒子是否收到错误“未指定许可证”？如果是这样，请在结帐的 Cargo.toml 中的“[package]”下添加“publish = false”。
- 您是否收到依赖项的错误“无法满足许可要求”？如果是，首先确定该项目拥有什么许可证，以及该系统是否足以满足该许可证的要求。如果您不确定，请咨询律师。验证该系统可接受后，将许可证的 SPDX 标识符添加到“script/licenses/zed-licenses.toml”中的“accepted”数组中。
- `cargo-about` 找不到依赖项的许可证？如果是这样，请在 `script/licenses/zed-licenses.toml` 的末尾添加一个说明字段，如 [cargo 书籍](https://embarkstudios.github.io/cargo-about/cli /generate /config.html#crate-configuration）。