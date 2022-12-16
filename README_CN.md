<h1 align="center">
  <img src="./md/icon.png" width="128" />
  <br>
  Lanaya
  <br>
</h1>

<h3 align="center">
一个简洁易用的剪切板管理
</h3>

<h3 align="center">
<a src="https://github.com/ChurchTao/Lanaya">English</a> |
<a scr="https://github.com/ChurchTao/Lanaya/blob/master/README_CN.md" target="_blank">中文</a>
</h3>

## 简介

`Lanaya` 来自于`DOTA2`中的圣堂刺客, 简洁易用，全键盘操作的剪切板管理工具

## 功能

- 通过关键词搜索
- 全快捷键操作

## 未完成

- [ ] 新增设置页面，开放一些参数为设置首选项
- [ ] 增加主题
- [x] 引入`taildwind`管理 css
- [ ] 整理代码结构
- [ ] 新增复制图片历史的功能

## 下载

从 [release](https://github.com/ChurchTao/Lanaya/releases) 中下载.

## 开发

你需要安装 `Rust` 和 `Nodejs`，详细步骤查看 [这里](https://tauri.studio/docs/getting-started/prerequisites) ，然后按如下命令进行开发

```shell
npm install
```

然后

```shell
cargo tauri dev
```

如果需要构建

```shell
cargo tauri build
```

## 截图

<div align="center">
  <img src="./md/demo1.png" alt="demo1" width="100%" />
  <img src="./md/demo2.png" alt="demo2" width="100%" />
  <img src="./md/demo3.png" alt="demo3" width="100%" />
</div>

## 建议

👏🏻 非常欢迎提`Issue`和`PR`！毕竟一个人的力量有限。

## 技术栈

`Lanaya` 基于如下技术栈：

- [tauri-apps/tauri](https://github.com/tauri-apps/tauri): Build smaller, faster, and more secure desktop applications with a web frontend.
- [vitejs/vite](https://github.com/vitejs/vite): Next generation frontend tooling. It's fast!
- [vue3](https://github.com/vuejs/core): An approachable, performant and versatile framework for building web user interfaces.

## License

Apache-2.0 license. See [License here](./LICENSE) for details.
