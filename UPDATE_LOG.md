# Changelog

All changes will be documented in this file.

## v1.1.8

### Feature

- CN
  - 新增功能，添加标签、搜索标签
- EN
  - Add function, add tag, search tag

## v1.1.7

- CN

  - 窗口出现时，会自动聚焦到搜索框 [#4](https://github.com/ChurchTao/Lanaya/pull/4)
  - 修复了当复制 html 标签时，列表框结果展示异常的问题 [#5](https://github.com/ChurchTao/Lanaya/pull/5)

- EN
  - When the window appears, it will automatically focus on the search box [#4](https://github.com/ChurchTao/Lanaya/pull/4)
  - Fixed the problem that the list box result display is abnormal when copying html tags [#5](https://github.com/ChurchTao/Lanaya/pull/5)

## v1.1.6

- CN
  - 新增引入了 `daisyui`
  - 升级了 `vite`
  - 修复了`限制条数`功能不生效的问题
- EN
  - Added `daisyui`
  - Upgraded `vite`
  - Fixed the problem that the `limit number` function does not take effect

## v1.1.5

- CN
  - 我忘了解开 `blur` 的注释..
- EN
  - I forgot to remove the comment of `blur`

## v1.1.4

### Bug Fixes

- CN
  - 修复了因为 `objc` 导致的内存泄露
  - 修复了显示速度
- EN
  - Fixed the memory leak caused by `objc`
  - Fixed the display speed

## v1.1.3

- CN
  - 修复了截取字符串时，不安全的切片导致的崩溃问题
- EN
  - Fixed the problem of unsafe slice caused by crash when cutting strings

## v1.1.2

### Feature

- CN
  - 新增`删除某一条`功能
- EN:
  - add `delete one` feature.

### Bug Fixes

- CN
  - 修复了大文本复制时，会导致 UI 渲染卡顿的问题
  - 修复了搜索收藏记录时，无法搜索到图片的问题
  - 修复了清空记录时，会清除收藏的记录的问题
- EN
  - Fixed the problem that large text copy will cause UI rendering to stall
  - Fixed the problem that the image cannot be searched when searching for favorite records
  - Fixed the problem that the record will be cleared when the record is cleared

## v1.1.1

### Bug Fixes

- CN
  - 修复了搜索时的 SQL 注入，导致崩溃的问题
- EN
  - Fixed the problem of SQL injection when searching, causing a crash

## v1.1.0

### Feature

- CN
  - 新增`收藏`功能
- EN:
  - add `favorite` feature.

### Bug Fixes

- CN
  - 修复复制 html 时，会导致显示问题
- EN
  - Fixed the problem that copying html will cause display problems

## v1.0.3

### Feature

- CN
  - 新增复制图片历史的功能
  - 使用`Rust`实现后台监听剪切板
  - 使用`Rust`实现`Sqlite`的数据库操作
- EN:
  - add copy image history.
  - use `Rust` to implement clipboard listener.
  - use `Rust` to implement `Sqlite` database

### Bug Fixes

- CN
  - 唤起窗口不限于主界面
  - 优化搜索高亮显示效果
  - 修复了复制文件时，会导致循环替换复制顺序的问题
- EN
  - Wake up the window is not limited to the main interface
  - Optimize the search highlight display effect
  - Fixed the problem that copying files will cause the loop replacement copy order

## v1.0.2

### Bug Fixes

- CN
  - 修复了复制文件时，会导致循环替换复制顺序的问题
- EN
  - Fixed the problem that copying files will cause the loop replacement copy order

## v1.0.1

### Bug Fixes

- CN
  - 修复了复制图片时，因 SCP 协议导致的图片无法显示的问题
- EN
  - Fixed the problem that the image could not be displayed due to the SCP protocol when copying the image

## v1.0.0

### Feature

- CN
  - 新增复制图片历史的功能
  - 使用`Rust`实现后台监听剪切板
  - 使用`Rust`实现`Sqlite`的数据库操作
- EN:
  - add copy image history.
  - use `Rust` to implement clipboard listener.
  - use `Rust` to implement `Sqlite` database

### Bug Fixes

- CN
  - 唤起窗口不限于主界面
  - 优化搜索高亮显示效果
- EN
  - Wake up the window is not limited to the main interface
  - Optimize the search highlight display effect

## v0.1.1

### Feature

- 实装 限制剪切板记录条数

### Bug Fixes

- 主界面失去焦点时，不会自动关闭界面

## v0.1.0

### Feature

- 实装 多语言
- 实装 自动更新
- 实装 快捷键设置
- 实装 开机启动

## v0.0.9

### Feature

- 这是一个测试版本，新增了，设置页面，自动版本更新（手动检测的还没写好），自动打包 ci，以及一些交互优化，过年了，先这样啦~
- This is a test version, with new additions, settings page, automatic version update (manual detection has not yet been written), automatic packaging ci, and some interactive optimizations, Chinese New Year, let’s do this first~
- 实装 快捷键设置
- 实装 开机启动

## v0.0.8

### Feature

- 新增了配置、关于两个页面，功能未全部实现
- 优化主页面的交互

## v0.0.1

### Documentation

- Base function complete.
