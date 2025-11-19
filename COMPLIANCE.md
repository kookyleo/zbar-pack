# LGPL 合规说明

## 概述

**zbar-pack** 在 `vendored` 模式下静态链接 ZBar 库 (LGPLv2.1+)。本文档说明如何满足 LGPL 许可证的合规要求。

## ZBar 许可证信息

- **许可证**: GNU Lesser General Public License v2.1 or later (LGPLv2.1+)
- **版本**: 0.23.93
- **来源**: https://github.com/mchehab/zbar
- **提交哈希**: bb05ec54eec57f8397cb13fb9161372a281a1219

## LGPL 静态链接要求

根据 LGPLv2.1 第 6 条,静态链接 LGPL 库的应用必须:

### 1. 提供源码访问

✅ **已满足**:
- ZBar 完整源码随本 crate 分发,位于 `zbar-src/vendor/zbar-0.23.93/`
- 上游源码可从 https://github.com/mchehab/zbar 获取
- 无任何修改或补丁应用于上游代码

### 2. 允许用户替换库

✅ **已满足**:

**方法 1: 使用 system 模式**

用户可以通过 `system` feature 使用自己编译的 ZBar 库:

```toml
[dependencies]
zbar-pack = { version = "0.1", default-features = false, features = ["system"] }
```

然后设置环境变量指向自定义库:

```bash
export PKG_CONFIG_PATH=/path/to/custom/zbar/lib/pkgconfig
cargo build
```

**方法 2: 替换 vendored 源码**

用户可以替换 `zbar-src/vendor/zbar-0.23.93/` 目录中的源码为自己修改的版本,然后重新构建。

### 3. 提供重新链接机制

✅ **已满足**:

详细的重新链接步骤见 [RELINKING.md](RELINKING.md)

### 4. 附带许可证和版权声明

✅ **已满足**:

- ZBar 许可证: `zbar-src/vendor/zbar-0.23.93/LICENSE.md`
- ZBar 版权: `zbar-src/vendor/zbar-0.23.93/COPYING`
- 分发二进制文件时,上述文件必须包含在发行包中

## 源码修改记录

### 当前状态

截至 0.1.0 版本,本项目**未对 ZBar 源码做任何修改**。

### 如果未来有修改

如果未来版本对 ZBar 源码进行修改,我们将:

1. 在本文档中记录所有修改
2. 提供修改补丁文件
3. 按 LGPL 要求公开修改后的源码

## 合规检查清单

分发使用 zbar-pack 的应用时,请确保:

- [ ] 包含本项目的 LICENSE 文件 (Apache-2.0)
- [ ] 包含 ZBar 的 LICENSE.md (LGPLv2.1+)
- [ ] 在文档中说明使用了 ZBar 库
- [ ] 提供获取 ZBar 源码的方式 (例如链接到本仓库或上游)
- [ ] 说明如何使用 `system` feature 替换 ZBar 库
- [ ] 如果修改了 ZBar 源码,提供修改后的完整源码

## 版本跟踪

| zbar-pack 版本 | ZBar 版本 | ZBar 提交哈希 | 修改? | 补丁文件 |
|----------------|-----------|---------------|-------|----------|
| 0.1.0          | 0.23.93   | bb05ec54      | 否    | N/A      |

## 安全公告响应

我们承诺:

1. 监控 ZBar 上游的安全公告
2. 在 CVE 发布后 7 天内发布更新版本
3. 在发布说明中明确标注安全修复

用户可通过 `system` feature 快速获得发行版提供的安全补丁:

```bash
# 使用系统 ZBar 库 (通常已打补丁)
cargo build --no-default-features --features system
```

## 商业使用

LGPL 允许商业使用,但需遵守上述要求:

- ✅ 可以在专有软件中使用
- ✅ 可以静态链接
- ❗ 必须允许用户替换 LGPL 库部分
- ❗ 必须提供 LGPL 库的源码访问

## 联系方式

如有合规相关问题,请联系:

- 作者: kookyleo <kookyleo@gmail.com>
- Issue 追踪: https://github.com/kookyleo/zbar-pack/issues

## 参考资料

- [GNU LGPL v2.1 全文](https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html)
- [LGPL 使用指南](https://www.gnu.org/licenses/gpl-faq.html#LGPLStaticVsDynamic)
- [ZBar 许可证](https://github.com/mchehab/zbar/blob/master/LICENSE.md)

## 免责声明

本文档提供的合规信息仅供参考,不构成法律建议。如有具体法律问题,请咨询专业律师。
