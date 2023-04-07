# Use Rust built `create-vue-monorepo` cli

> 使用Rust重构`create-vue-monorepo` cli 库，原库是使用ESM构建的

| 技术栈 | 版本 |
| :-- | :-- |
| rust | 1.67.1 |
| dialoguer | 0.10.3 |
| console | 0.15.5 |
| tempfile | 3.5.0 |


## 思路

  1. 通过命令行交互创建vue monorepo 项目（pnpm workspace）
  2. 填写项目名称、选择需要添加的配置
  3. 根据所选择的配置渲染相应的模板
  4.  ...

## 目前困难

1. 使用Rust 操作文件及目录
2. 文件渲染
3. 命令打包等
4. 是否转为npm命令
