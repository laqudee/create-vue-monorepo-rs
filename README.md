# Use Rust built `create-vue-monorepo` cli

> 使用Rust重构`create-vue-monorepo` cli 库，原库是使用ESM构建的

| 技术栈 | 版本 |
| :-- | :-- |
| rust | 1.67.1 |
| dialoguer | 0.10.3 |
| console | 0.15.5 |
| termcolor | 1.2.0 |
| regex | 1.7.3 |
| serde_json | 1.0.159 |
| git2 | 0.17.0 |


## 开发指南

```shell
git clone https://github.com/laqudee/create-vue-monorepo-rs.git

cd create-vue-monorepo-rs

cargo install
# or 
cargo build

cargo run
```

## 构建过程

1. 首先将模板`template`放到根目录下，分为`base`，`code`， `config`这三个子目录，根据选择的配置不同，进行渲染。
2. 使用`dialoguer`库，生成命令行配置选择
3. 根据选择的配置，执行`render()`函数进行渲染
4. 使用`git2`库初始化git
5. 渲染完成，输出提示信息
6. 退出程序

## 目前困难

1. [x] ~~使用Rust 操作文件及目录~~
2. [x] ~~文件渲染~~
3. [ ] 命令打包的方式及提供几种构建方式
4. [ ] 是否转为npm命令
