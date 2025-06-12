# Rust实现的简易Git系统实验报告

## 项目简介

本项目使用Rust编程语言实现了一个简易的Git版本控制系统，支持Git的核心功能如仓库管理、文件追踪、提交历史和分支操作等。项目重点在于理解Git内部工作机制，并通过Rust实现其核心功能。

**开发者**：谭泽晖（221840188）

[Karenthepumpkin/rust-git](https://github.com/Karenthepumpkin/rust-git)

## 项目结构

### 主要目录结构

```
src/
├── cli/         命令行参数解析 
├── core/        Git核心逻辑 
│   ├── repository/   仓库管理 
│   ├── index/        缓存区维护 
│   ├── object/       Git对象管理 
│   ├── reference/    引用管理 
│   ├── tree/         tree文件处理 
│   ├── commit/       commit文件处理 
│   └── blob/         blob文件处理 
├── utils/       辅助模块 
│   ├── hash/         哈希计算 
│   └── error/        错误处理 
└── test/        测试脚本 
```

### 详细文件结构

```plaintext
src
├── cli
│   ├── args.rs
│   ├── commands
│   │   ├── add.rs
│   │   ├── branch.rs
│   │   ├── checkout.rs
│   │   ├── commit.rs
│   │   ├── init.rs
│   │   ├── merge.rs
│   │   └── rm.rs
│   └── mod.rs
├── core
│   ├── blob.rs
│   ├── commit.rs
│   ├── index.rs
│   ├── mod.rs
│   ├── object.rs
│   ├── reference.rs
│   ├── repository.rs
│   └── tree.rs
├── utils
│   ├── error.rs
│   ├── hash.rs
│   └── mod.rs
├── main.rs
test/
.gitignore
Cargo.toml
Cargo.lock
```

## 功能实现

支持的核心Git功能包括：

```
仓库初始化 (git init)
- 创建基础Git目录结构
- 初始化HEAD引用

文件操作 (git add, git rm)
- 添加文件到暂存区
- 从暂存区移除文件
- 创建blob对象

提交变更 (git commit)
- 创建tree对象（记录目录结构）
- 创建commit对象（记录提交信息）
- 更新HEAD引用

分支管理
- 分支创建与切换 (git checkout)
- 分支合并 (git merge)
- 冲突检测与解决
```

## 核心模块介绍

1. **objects.rs**  
   实现Git的核心对象模型  
   ```
   - 支持blob、tree和commit三种对象类型
   - 负责管理.git/object下文件的创建
   ```

2. **index.rs**  
   模拟Git的暂存区(index)结构  
   
   ```
   - 跟踪工作区文件状态
   - 管理文件路径与哈希值的映射关系
   ```
   
3. **repository.rs**  
   仓库的路径管理  
   
   ```
   - 对象存储与检索逻辑
   - 仓库初始化与状态检查
   - Repository下有path、blob_processor、index、tree、commit、reference等域，一切外部操作通过调用Repository对应函数进行
   ```
   
4. **reference.rs**  
   HEAD管理  
   
   ```
   - 分支创建、删除与切换
   - 提交历史追踪
   ```



## 实现细节与优化

### Tree文件实现

```
- 与传统Git不同，本项目采用扁平化存储
- 每一个Tree文件保存整个提交的所有文件路径/哈希值对而非单一的一个文件夹
- 优势：简化提交操作实现
- 缺点：存储空间利用率较低
```

### Index文件优化

```
- 存储整个项目文件路径/哈希值对
- 而非仅存储暂存区变更文件
- 简化提交逻辑，但增加了存储开销
```

## 测试与验证

```
测试场景：
- 仓库初始化与基础操作
- 多分支开发与合并
- 冲突检测与解决
```

## 已知问题与改进方向

### 当前问题

```
- rm命令后暂存的无法被引用的blob对象未被清理
- 大文件处理可能导致内存溢出
- merge命令测试覆盖不足，存在merge的相关未辨明的bug
- 模块边界不清晰，部分功能耦合，repository和index模块承担过多职责
- String与&str类型使用不一致
```

### 优化方向

```
架构优化
- 重构模块职责，提高复用性
- 解耦repository和index模块

性能改进
- 流式处理大文件，避免全内存加载
- 增量存储策略，减少空间占用

功能完善
- 实现blob垃圾回收机制
- 增强merge冲突处理能力
- 支持更多Git命令（如diff, log等）
```

## 总结与展望

### 项目收获

```
- 深入理解Git内部工作机制
  - 对象模型（blob, tree, commit）
  - 引用与分支管理
  - 合并算法与冲突解决

- Rust系统编程实践
  - 内存安全与所有权模型应用
  - 错误处理与模块化设计
  - 性能优化技巧
```

### 未来计划

```
远程仓库支持
- 实现push/fetch协议
- 支持远程分支管理

用户界面改进
- 开发图形界面(GUI)
- 或实现Web交互界面

工程化完善
- 提升测试覆盖率
- 完善文档体系
- 性能基准测试

```

本项目通过Rust实现了Git的核心功能，既加深了对版本控制系统原理的理解，也提升了Rust系统编程能力。后续将持续完善功能、优化性能并提升用户体验。
