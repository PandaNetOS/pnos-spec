# pnos-spec

pnos 系统级标准与共享库。所有 pnos 应用必须遵守本仓库定义的规范。

## 内容

- **系统级标准文档**：应用描述格式、容器规范、API 规范、配置标准、日志标准
- **`pnos` crate**：Rust 共享标准库，提供统一错误类型、响应格式、应用模型、容器配置、配置加载、结构化日志

## 模块

```
pnos::prelude::*          一行导入
├── error                 统一错误类型 + 错误码（7 大领域）
├── response              ApiResponse / ApiError / PageResult
├── app                   AppManifest (app.yml 模型)、AppStatus、HealthCheck
├── container             ContainerConfig、PortMapping、VolumeMount、EnvVar
├── protocol              API 路径常量、WebSocket 消息类型
├── config                PnosConfig（YAML + 环境变量覆盖）
├── logging               结构化日志初始化（tracing + env-filter）
├── time                  RFC3339 UTC 时间工具
└── utils                 字节格式化、UUID 校验等
```

## 使用

在 `Cargo.toml` 中添加路径依赖（与 pnos-spec 同级目录）：

```toml
[dependencies]
pnos = { path = "../pnos-spec" }
```

代码中：

```rust
use pnos::prelude::*;

fn main() {
    let resp: ApiResponse<String> = ApiResponse::success("hello".to_string());
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}
```

## 错误码体系

| 领域 | 范围 |
|------|------|
| 通用 | 0-999 |
| 容器 | 1000-1999 |
| 应用商店 | 2000-2999 |
| 文件 | 3000-3999 |
| 系统 | 4000-4999 |
| 网络 | 5000-5999 |

## 许可证

MIT
