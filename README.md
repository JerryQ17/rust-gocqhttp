<h1 style="text-align: center;">rust-gocqhttp</h1>

<div style="text-align: center;"><img src="https://img.shields.io/github/license/JerryQ17/rust-gocqhttp" alt=""><img src="https://img.shields.io/github/actions/workflow/status/JerryQ17/rust-gocqhttp/test.yml?label=cargo%20test" alt=""></div>

## 简介

> 由于QQ官方针对协议库的围追堵截，`go-cqhttp`的开发者无力继续维护其项目，在未来 `sign-server` 方案彻底被官方封死之后 `go-cqhttp` 将无法继续使用。同时`NTQQ`的出现让我们可以使用官方 **完美** 实现的协议实现来继续开发Bot, 不再担心由于协议实现不完美而导致被识别.
> 我们建议所有QQBot项目开始做好迁移至无头`NTQQ`或类似基于官方客户端技术的准备以应对未来的彻底封锁。
>
> 详见[QQ Bot的未来以及迁移建议 · Issue #2471 · Mrs4s/go-cqhttp (github.com)](https://github.com/Mrs4s/go-cqhttp/issues/2471)。

`rust-gocqhttp`是一个基于Rust语言，为[`go-cqhttp`](https://github.com/Mrs4s/go-cqhttp)开发的SDK。

所有的`CQCode`，`API`的命名均与`go-cqhttp`原生实现保持一致。

## 支持

### 通信协议

> PS：开发中

- [ ] 正向 HTTP
- [ ] 反向 HTTP
- [ ] 正向 WebSocket
- [ ] 反向 WebSocket

### 消息格式

- [x] 字符串
- [x] 数组
