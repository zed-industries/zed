# Zed Lite 技术规范文档

## 项目概述

Zed Lite 是基于 Zed 编辑器核心框架构建的轻量级代码编辑器入口点。它保留了 Zed 的完整 UI 框架和窗口管理系统，同时提供了一个简化的启动入口，专注于提供核心编辑功能。

## 架构设计

### 1. 项目结构

```
crates/zed_lite/
├── Cargo.toml          # 项目配置和依赖管理
├── src/
│   ├── main.rs         # 主入口点，应用初始化
│   └── zed_lite.rs     # 窗口配置和选项
└── resources/          # 应用资源文件
```

### 2. 核心组件架构

#### 2.1 应用初始化流程

```rust
main() -> Application::new() -> app.run() -> create_empty_workspace()
```

**初始化顺序：**
1. 解析命令行参数
2. 初始化路径和目录结构
3. 设置日志系统
4. 创建 GPUI Application
5. 初始化核心组件
6. 创建空工作区

#### 2.2 依赖组件

**核心依赖：**
- `gpui`: UI 框架和窗口管理
- `workspace`: 工作区管理
- `editor`: 编辑器核心功能
- `theme`: 主题系统
- `settings`: 配置管理
- `client`: 网络客户端
- `call`: 通话系统（工作区依赖）
- `title_bar`: 标题栏组件

**支持依赖：**
- `assets`: 资源管理
- `fs`: 文件系统抽象
- `language`: 语言支持
- `session`: 会话管理
- `node_runtime`: Node.js 运行时

### 3. 技术实现细节

#### 3.1 HTTP 客户端配置

```rust
// 设置用户代理
let user_agent = format!(
    "ZedLite/{} ({}; {})",
    AppVersion::global(cx),
    std::env::consts::OS,
    std::env::consts::ARCH
);

// 创建 HTTP 客户端
let http = ReqwestClient::user_agent(&user_agent)
    .expect("could not start HTTP client");
cx.set_http_client(Arc::new(http));

// 设置客户端的 HTTP 客户端
cx.set_http_client(client.http_client());
```

#### 3.2 窗口配置

```rust
WindowOptions {
    titlebar: Some(TitlebarOptions {
        title: Some("Zed Lite".into()),
        appears_transparent: true,
        traffic_light_position: Some(point(px(9.0), px(9.0))),
    }),
    window_min_size: Some(gpui::Size {
        width: px(640.0),
        height: px(480.0),
    }),
    // ... 其他配置
}
```

#### 3.3 工作区创建

```rust
workspace::open_new(
    Default::default(),
    app_state,
    cx,
    |workspace, window, cx| {
        let restore_on_startup = WorkspaceSettings::get_global(cx).restore_on_startup;
        match restore_on_startup {
            workspace::RestoreOnStartupBehavior::Launchpad => {
                // 显示启动面板
            }
            _ => {
                // 创建新的空文件
                editor::Editor::new_file(workspace, &Default::default(), window, cx);
            }
        }
    },
)
```

## 编码规范

### 1. Rust 编码准则

遵循项目根目录 `.rules` 文件中定义的编码规范：

#### 1.1 错误处理
- **禁止使用 `unwrap()`**：使用 `?` 操作符传播错误
- **不要静默丢弃错误**：使用 `.log_err()` 或显式错误处理
- **异步操作错误传播**：确保错误能传播到 UI 层

```rust
// ❌ 错误示例
let result = some_operation().unwrap();
let _ = client.request(...).await?;

// ✅ 正确示例
let result = some_operation()?;
client.request(...).await?;
some_operation().log_err();
```

#### 1.2 代码组织
- **优先在现有文件中实现功能**：避免创建过多小文件
- **避免 `mod.rs` 路径**：使用 `src/module_name.rs`
- **使用完整单词**：避免缩写变量名

#### 1.3 异步编程
- **变量遮蔽用于作用域克隆**：
```rust
executor.spawn({
    let task_ran = task_ran.clone();
    async move {
        *task_ran.borrow_mut() = true;
    }
});
```

### 2. GPUI 框架规范

#### 2.1 上下文管理
- `App`: 根上下文类型，提供全局状态访问
- `Context<T>`: 实体更新时提供，解引用为 `App`
- `AsyncApp`: 异步上下文，可跨 await 点持有

#### 2.2 实体操作
```rust
// 读取实体
entity.read(cx)
entity.read_with(cx, |entity, cx| { ... })

// 更新实体
entity.update(cx, |entity, cx| { ... })
entity.update_in(cx, |entity, window, cx| { ... })
```

#### 2.3 并发模型
- **前台线程**：所有实体和 UI 渲染
- **后台任务**：使用 `cx.background_spawn()`
- **任务管理**：使用 `.detach()` 或存储 `Task<R>`

### 3. 构建和测试

#### 3.1 构建命令
```bash
# 构建 zed_lite
cargo build -p zed_lite

# 使用项目 clippy
./script/clippy
```

#### 3.2 测试规范
- **GPUI 测试**：使用 GPUI 执行器计时器
- **避免 `smol::Timer::after(...)`**：在依赖 `run_until_parked()` 时

## 性能和资源管理

### 1. 内存使用
- **当前内存占用**：约 168MB（优化后）
- **字体加载**：异步加载嵌入字体
- **资源管理**：使用 Arc 共享资源

### 2. 启动性能
- **初始化顺序优化**：按依赖关系顺序初始化组件
- **延迟加载**：非关键组件延迟初始化
- **并发初始化**：使用后台任务并行处理

## 配置和扩展

### 1. 命令行参数
```rust
#[derive(Parser, Debug)]
struct Args {
    /// 设置用户数据自定义目录
    #[arg(long, value_name = "DIR")]
    user_data_dir: Option<String>,
}
```

### 2. 环境变量
- `ZED_LITE_BUILD_ID`: 构建标识符
- `ZED_LITE_COMMIT_SHA`: 提交哈希
- `RUST_LOG`: 日志级别控制

### 3. 目录结构
```
~/.config/zed-lite/     # 配置目录
~/.local/share/zed-lite/ # 数据目录
~/.cache/zed-lite/      # 缓存目录
/tmp/zed-lite/          # 临时目录
```

## 部署和分发

### 1. 构建配置
```toml
[package]
name = "zed_lite"
version = "0.1.0"
edition = "2024"
license = "GPL-3.0-or-later"
authors = ["Zed Team <hi@zed.dev>"]
default-run = "zed_lite"
```

### 2. 平台支持
- **Windows**: 支持，使用 `windows_subsystem = "windows"`
- **macOS**: 支持，包含 traffic light 位置配置
- **Linux**: 支持，包含 Wayland 和 X11

### 3. 资源打包
- **图标资源**: `resources/app-icon*.png`
- **字体资源**: 嵌入 TTF 字体文件
- **主题资源**: 通过 Assets 系统加载

## 故障排除

### 1. 常见问题

#### HTTP 客户端错误
```
[ERROR client::user] No HttpClient available
```
**解决方案**: 确保正确设置 HTTP 客户端，按顺序调用：
1. `cx.set_http_client(Arc::new(http))`
2. `cx.set_http_client(client.http_client())`

#### 调用系统依赖错误
```
no state of type call::call_impl::GlobalActiveCall exists
```
**解决方案**: 在初始化 title_bar 之前初始化 call 系统：
```rust
call::init(client.clone(), user_store.clone(), cx);
title_bar::init(cx);
```

### 2. 调试技巧
- **启用日志**: `RUST_LOG=debug ./target/debug/zed_lite.exe`
- **内存监控**: 使用任务管理器监控内存使用
- **进程检查**: `tasklist | findstr zed_lite`

## 未来发展方向

### 1. 功能扩展
- **插件系统**: 支持轻量级插件
- **主题定制**: 简化的主题配置
- **快捷键定制**: 基础快捷键配置

### 2. 性能优化
- **启动时间**: 进一步优化初始化流程
- **内存占用**: 减少不必要的组件加载
- **响应性**: 优化 UI 渲染性能

### 3. 用户体验
- **简化配置**: 提供更简单的配置界面
- **错误提示**: 改进错误信息显示
- **文档完善**: 提供用户使用指南

## 版本历史

### v0.1.0 (当前版本)
- ✅ 基础 Zed 框架集成
- ✅ 空工作区创建
- ✅ HTTP 客户端配置
- ✅ 标题栏和窗口管理
- ✅ 基础编辑器功能

### 计划版本
- **v0.2.0**: 插件系统基础
- **v0.3.0**: 配置界面优化
- **v1.0.0**: 稳定版本发布

---

*本文档遵循 Zed 项目的技术规范和编码准则，定期更新以反映项目的最新状态。*