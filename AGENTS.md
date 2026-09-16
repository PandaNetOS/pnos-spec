# pnos-spec AGENTS.md

> 本文件是 AI 代理进入 pnos-spec 仓库时的首读指南。
> 生态级全局约束请参考 [根目录 AGENTS.md](../AGENTS.md)。

## 仓库定位

pnos-spec 是 PandaNetOS 生态的**系统级标准库**，所有项目直接依赖（git）。统一协议/类型/错误码/配置规范，同时包含系统级标准文档和模板。

## 架构概览

```
pnos-spec/
├── src/            # Rust 标准库代码
│   ├── lib.rs      # 库入口
│   ├── error.rs    # 统一错误类型 + 错误码（7 大领域）
│   ├── response.rs # ApiResponse / ApiError / PageResult
│   ├── app.rs      # AppManifest、AppStatus、HealthCheck
│   ├── config.rs   # PnosConfig（YAML + 环境变量覆盖）
│   ├── logging.rs  # 结构化日志初始化（tracing + env-filter）
│   ├── protocol.rs # API 路径常量、WebSocket 消息类型
│   ├── registry.rs # 组件注册
│   ├── events.rs   # 事件协议
│   ├── task.rs     # 任务协议
│   ├── capability.rs # 能力自描述
│   ├── component.rs  # 组件模型
│   ├── discovery.rs  # 服务发现
│   ├── container.rs  # 容器配置
│   ├── proxy.rs      # 反向代理
│   ├── system.rs     # 系统信息
│   ├── time.rs       # RFC3339 UTC 时间工具
│   └── utils.rs      # 字节格式化、UUID 校验等
├── scripts/        # 脚本（check_compliance.sh）
├── actions/        # GitHub Actions（tag-guard）
├── templates/      # 标准模板（AGENTS.md.template）
└── Cargo.toml
```

## 构建与测试

| 命令 | 说明 |
|---|---|
| `cargo build --release` | 构建标准库 |
| `cargo test --all` | 运行所有测试 |
| `cargo fmt --all -- --check` | 格式检查 |
| `cargo clippy --all-targets -- -D warnings` | 静态分析 |

## 依赖关系

- **依赖**：无外部生态依赖（最底层）
- **被依赖**：所有生态项目（pdc/pk/spde/pnos-runtime/pnos-sdk 等）

## 注意事项

1. pnos-spec 是最底层标准库，**不允许依赖其他生态项目**
2. 新增类型/错误码/协议需保持向后兼容
3. 配置字段必须带 `#[serde(default)]` 和默认值函数
4. `templates/` 目录存放生态通用模板（AGENTS.md、config.yaml 等）

## 变更历史

| 日期 | 版本 | 变更内容 |
|---|---|---|
| 2026-09-16 | v1.0 | 初始版本，新增 templates/AGENTS.md.template |
