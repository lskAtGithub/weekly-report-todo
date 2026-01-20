# Weekly Report Todo

一个基于 Rust 的命令行待办事项管理工具，支持按周/月时间范围筛选任务，适用于工作周报和日常任务管理。

## 功能特性

- ✅ 添加、完成、删除待办事项
- 📅 支持按周和月时间范围筛选任务
- 💾 数据持久化存储到 JSON 文件
- 🕐 自动记录任务创建时间
- 📊 支持查看已完成和所有任务状态

## 项目结构

```
weekly-report-todo/
├── src/
│   ├── main.rs          # 命令行接口和主程序
│   └── todo.rs          # TodoList 核心逻辑和数据结构
├── Cargo.toml           # 项目配置和依赖
├── Cargo.lock           # 依赖版本锁定
├── todos.json           # 数据存储文件（自动生成）
└── README.md            # 项目说明文档
```

## 核心模块说明

### main.rs
- 命令行参数解析和处理
- 用户交互界面
- 命令分发和执行

### todo.rs
- `Todo` 结构体：单个待办事项的数据模型
- `TodoList` 结构体：待办事项集合管理
- `TimeRange` 枚举：时间范围筛选（周/月）
- `Status` 枚举：任务状态筛选（全部/已完成）

## 安装和运行

### 前提条件
- 安装 Rust 工具链 (rustc, cargo)

### 构建项目
```bash
# 克隆项目
cd weekly-report-todo

# 构建项目
cargo build --release

# 运行程序
cargo run
```

## 使用方法

### 基本命令

```bash
# 添加待办事项
cargo run add "完成周报编写"

# 标记任务完成
cargo run done 1

# 删除任务
cargo run del 1

# 查看所有任务
cargo run list
```

### 命令说明

| 命令 | 参数 | 说明 |
|------|------|------|
| `add` | `<title>` | 添加新的待办事项 |
| `done` | `<id>` | 标记指定ID的任务为已完成 |
| `del` | `<id>` | 删除指定ID的任务 |
| `list` | - | 显示所有待办事项 |

## 数据存储

程序使用 JSON 格式存储数据，文件名为 `todos.json`，包含以下字段：

```json
{
  "todos": [
    {
      "id": 1,
      "title": "示例任务",
      "completed": false,
      "created_at": "2024-01-20T10:30:00Z"
    }
  ]
}
```

## 技术栈

- **Rust 2024 Edition** - 编程语言
- **serde** - 序列化/反序列化库
- **serde_json** - JSON 处理
- **chrono** - 日期时间处理

## 开发说明

### 项目依赖
在 `Cargo.toml` 中定义：

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### 核心功能

1. **任务管理**：添加、完成、删除任务
2. **时间筛选**：按周或月范围筛选任务
3. **状态管理**：支持查看全部或已完成任务
4. **数据持久化**：自动保存到 JSON 文件

### 扩展功能

当前版本支持按时间范围筛选任务，可通过修改 `filter_by_range` 方法实现更多筛选条件。

## 许可证

MIT License