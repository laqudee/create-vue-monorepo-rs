# Use Rust built `create-vue-monorepo` cli

- A tool to quickly build a Vue workspace project managed by pnpm.

> 使用 Rust 重构`create-vue-monorepo` cli 库，原库是使用 ESM 构建的

| 技术栈     | 版本    |
| :--------- | :------ |
| rust       | 1.67.1  |
| dialoguer  | 0.10.3  |
| console    | 0.15.5  |
| termcolor  | 1.2.0   |
| regex      | 1.7.3   |
| serde_json | 1.0.159 |
| git2       | 0.17.0  |

## 使用

> 目前仅支持`cargo install`安装方式

- 下载安装

```shell
cargo install create-vue-monorepo-rs
```

- 使用

```shell
# 输入create-vue-monorepo-rs，按回车键即可
create-vue-monorepo-rs
```

![演示](./media/it_work.png)

## 开发指南

- 下载&安装依赖

```shell
git clone https://github.com/laqudee/create-vue-monorepo-rs.git

cd create-vue-monorepo-rs

cargo install

cargo run
```

- 构建正式包

```shell
cargo build --release
```

- 发布到`crates.io`

```shell
cargo publish
```

## 构建过程

1. 首先将模板`template`放到根目录下，分为`base`，`code`， `config`这三个子目录，根据选择的配置不同，进行渲染。
2. 使用`dialoguer`库，生成命令行配置选择
3. 根据选择的配置，执行`render()`函数进行渲染
4. 使用`git2`库初始化 git
5. 渲染完成，输出提示信息
6. 退出程序

## 待解决的问题

1. [x] ~~使用 Rust 操作文件及目录~~
2. [x] ~~文件渲染~~
3. [ ] 命令打包的方式及提供几种构建方式
   - 目前支持`cargo install creata-vue-monorepo-rs`方式安装使用
4. [ ] `git2`库没有生效
5. [ ] 生成的`package.json`内容按照字母顺序排序了，不符合正常的`package.json`顺序
6. [ ] 是否转为 npm 命令
