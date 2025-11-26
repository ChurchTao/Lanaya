<h1 align="center">
  <img src="./md/icon.png" width="128" />
  <br>
  Lanaya
  <br>
</h1>

<h3 align="center">
一个简洁易用的剪贴板管理
</h3>

<div align="center">
<img alt="GitHub all releases" src="https://img.shields.io/github/downloads/churchTao/Lanaya/total?color=%23">
<img alt="GitHub" src="https://img.shields.io/github/license/churchtao/lanaya?color=%23">
<img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/churchtao/lanaya?style=social">
</div>

<h3 align="center">
<a href="https://github.com/ChurchTao/Lanaya/blob/master/README_EN.md">English</a> |
<a href="https://github.com/ChurchTao/Lanaya/blob/master/README_CN.md" target="_blank">中文</a>
</h3>

## 简介

`Lanaya` 来自于`DOTA2`中的圣堂刺客, 简洁易用，全键盘操作的剪贴板管理工具

## 提示

因精力不足，该项目已经不再升级维护，仅作学习参考使用

## 功能

- 通过关键词搜索
- 全快捷键操作
- 设置历史条数范围
- 多语言
- 自动更新
- 输入 `f:xxx` 搜索收藏的记录
- 输入 `t:xxx` 搜索标签分类

## 未完成

- [x] 引入`taildwind`管理 css
- [x] 新增复制图片历史的功能
- [ ] 增加主题
- [x] 新增收藏夹功能
- [ ] 增加`Windows`,`Linux`的适配
- [x] 使用`Rust`实现后台监听剪切板
- [x] 使用`Rust`实现`Sqlite`的数据库操作

## 下载

从 [release](https://github.com/ChurchTao/Lanaya/releases) 中下载.

### Mac 用户

如果提示`软件已损坏，请移到废纸篓`，可以使用命令 `xattr -cr /Applications/Lanaya.app` 解决

## 开发

你需要安装 `Rust` 和 `Nodejs`，详细步骤查看 [这里](https://tauri.app/zh-cn/v1/guides/getting-started/prerequisites) ，然后按如下命令进行开发

```shell
npm install
```

然后

```shell
cargo install tauri-cli # 提示没有 cargo tauri 命令需先执行安装
cargo tauri dev
```

如果需要构建

```shell
cargo tauri build
```

## 截图

<div align="center">
  <img src="./md/demo1.png" alt="demo1" width="80%" />
  <img src="./md/demo2.png" alt="demo2" width="80%" />
  <img src="./md/demo3.png" alt="demo3" width="80%" />
  <img src="./md/demo4.png" alt="demo4" width="80%" />
  <img src="./md/demo5.png" alt="demo4" width="80%" />
  <img src="./md/demo6.png" alt="demo6" width="80%" />
  <img src="./md/demo7.png" alt="demo7" width="80%" />
</div>

## 建议

👏🏻 非常欢迎提`Issue`和`PR`！毕竟一个人的力量有限。

## 技术栈

`Lanaya` 基于如下技术栈：

- [tauri-apps/tauri](https://github.com/tauri-apps/tauri): Build smaller, faster, and more secure desktop applications with a web frontend.
- [vitejs/vite](https://github.com/vitejs/vite): Next generation frontend tooling. It's fast!
- [vue3](https://github.com/vuejs/core): An approachable, performant and versatile framework for building web user interfaces.
- [tailwindlabs](https://github.com/tailwindlabs) Creators of Tailwind CSS and Headless UI, and authors of Refactoring UI.

## License

Apache-2.0 license. See [License here](./LICENSE) for details.
