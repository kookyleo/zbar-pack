# zbar-pack

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/zbar-pack.svg)](https://crates.io/crates/zbar-pack)

**zbar-pack** 是一个 ZBar 条码扫描库的 Rust 包装,提供静态链接和跨平台支持。

## 特性

- ✅ **静态链接**: 默认编译并静态链接内置的 ZBar 源码,无需系统依赖
- ✅ **musl 构建**: 完整支持 musl 目标,便于创建完全静态的可执行文件
- ✅ **跨平台**: 支持 Linux (glibc/musl)、macOS、Windows (MSVC/GNU)
- ✅ **安全 API**: 提供符合 Rust 习惯的安全高层 API
- ✅ **灵活配置**: 通过 feature flags 控制编解码器和链接方式
- ✅ **许可证合规**: 完整的 LGPL 合规说明和源码可替换性

## 快速开始

### 安装

在 `Cargo.toml` 中添加:

```toml
[dependencies]
zbar-pack = "0.1"
```

### 基础用法

```rust
use zbar_pack::{ImageScanner, Image, SymbolType};

fn main() {
    // 创建扫描器
    let mut scanner = ImageScanner::new().unwrap();

    // 配置扫描器 (可选)
    scanner.set_config(SymbolType::QRCODE, 0, 1).unwrap();

    // 从灰度图像数据创建图像
    let image = Image::from_gray(&data, width, height).unwrap();

    // 扫描图像
    let symbols = scanner.scan_image(&image).unwrap();

    // 遍历识别到的符号
    for symbol in symbols {
        println!("类型: {:?}", symbol.symbol_type());
        println!("数据: {}", symbol.data());
        println!("质量: {}", symbol.quality());
    }
}
```

## Feature Flags

### 链接模式

- `vendored` (默认): 编译并静态链接内置的 ZBar 源码
- `system`: 使用 pkg-config 查找系统安装的 ZBar 库
- `dynamic`: 动态链接系统 ZBar 库

### 编解码器

- `all-codecs` (默认): 启用所有编解码器
- `minimal`: 最小化构建,需手动启用需要的编解码器
- `codec-qrcode`: QR 码
- `codec-ean`: EAN/UPC 系列
- `codec-code128`: Code 128
- `codec-code93`: Code 93
- `codec-code39`: Code 39
- `codec-codabar`: Codabar
- `codec-i25`: Interleaved 2 of 5
- `codec-databar`: GS1 DataBar
- `codec-sqcode`: SQ 码

### 示例配置

```toml
# 仅支持 QR 码的最小化构建
[dependencies]
zbar-pack = { version = "0.1", default-features = false, features = ["vendored", "codec-qrcode"] }

# 使用系统 ZBar 库
[dependencies]
zbar-pack = { version = "0.1", default-features = false, features = ["system"] }
```

## 构建要求

### vendored 模式 (默认)

- Rust 1.70+
- C 编译器 (gcc/clang/msvc)
- 无需系统 ZBar 库

### system 模式

- Rust 1.70+
- 系统已安装 ZBar 库 (版本 >= 0.10)
- pkg-config (Linux/macOS)

### musl 静态构建

```bash
# 安装 musl 工具链
rustup target add x86_64-unknown-linux-musl

# 编译静态可执行文件
cargo build --release --target x86_64-unknown-linux-musl
```

## 许可证与合规

### 本项目许可证

- **zbar-pack** (本 Rust 包装): Apache-2.0

### ZBar 上游许可证

- **ZBar 库**: LGPLv2.1+

### LGPL 静态链接合规

本项目在 `vendored` 模式下静态链接 ZBar (LGPLv2.1+),为满足 LGPL 要求:

1. **源码可获取性**
   - ZBar 完整源码位于 `zbar-src/vendor/zbar-0.23.93/`
   - 上游地址: https://github.com/mchehab/zbar
   - 版本: 0.23.93

2. **库可替换性**
   - 使用 `system` feature 可切换到系统 ZBar 库
   - 提供编译脚本和说明允许用户重新编译链接

3. **版本对齐承诺**
   - 定期与上游 ZBar 对齐版本
   - 快速响应安全公告 (CVE)

4. **完整文档**
   - 详见 [COMPLIANCE.md](COMPLIANCE.md)
   - 重新链接指南见 [RELINKING.md](RELINKING.md)

### 何时使用 system 模式

建议在以下场景使用 `system` 模式:

- 企业发行版策略要求使用系统库
- Linux 发行版打包
- 需要快速获得发行版安全补丁
- 简化合规流程

## 平台差异说明

### Linux

- **glibc**: 支持动态和静态链接,但静态链接 glibc 存在已知问题,不推荐
- **musl**: 推荐用于全静态构建,兼容性好

### macOS

- 使用 Clang 和系统 SDK
- 自动处理 rpaths

### Windows

- 支持 MSVC 和 GNU 工具链
- MSVC: 使用 UCRT
- GNU: 使用 MSVCRT

## 包体积控制

为符合 crates.io 的体积限制:

- 使用精简的 ZBar 源码快照
- 不包含完整 git 历史
- 排除测试资源和示例图片
- 移除不必要的平台特定代码

## 故障排除

### 编译失败

```bash
# 清理构建缓存
cargo clean

# 尝试使用 system 模式
cargo build --no-default-features --features system
```

### musl 链接错误

确保使用正确的目标三元组:
```bash
cargo build --target x86_64-unknown-linux-musl
```

### Windows MSVC 问题

确保安装了 Visual Studio Build Tools 或完整的 Visual Studio。

## 贡献

欢迎提交 Issue 和 Pull Request!

## 相关项目

- [zbar](https://github.com/mchehab/zbar) - 上游 ZBar C 库
- [zbar-rust](https://github.com/magiclen/zbar-rust) - 另一个 Rust 绑定

## 变更日志

详见 [CHANGELOG.md](CHANGELOG.md)

## 作者

kookyleo <kookyleo@gmail.com>

## 许可证

Apache-2.0 (本 Rust 包装)

ZBar 库遵循 LGPLv2.1+ 许可证,详见 `zbar-src/vendor/zbar-0.23.93/LICENSE.md`
