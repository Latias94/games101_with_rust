# 用 Rust 重温 GAMES101

[GAMES101-现代计算机图形学入门-闫令琪](https://www.bilibili.com/video/BV1X7411F744/)

这个框架是**非官方的**，你可以用这个作业框架来用 `Rust` 写作业当练习。代码在各个 assignmentX 文件夹中，其中 `todo!()`
是作业需要写代码的地方。

代码和原本作业框架的差别是：这个框架摆脱了 `OpenCV` 的依赖，也没有使用 `SDL2` 库，Windows 下安装基本的 `Rust` 环境后能直接运行。

窗口库使用 [emoon/rust_minifb](https://github.com/emoon/rust_minifb)，支持 macOS, Linux and Windows (64-bit and
32-bit)。

我的作业实现在 examples 文件夹下，你可以运行 `cargo run --example hw1`
来执行窗口，或 `cargo r --example hw1 -- -r 0 output.png` 来生成图像。

## 开写作业！

安装 [Rust](https://www.rust-lang.org/learn/get-started).

```shell
git clone https://github.com/latias94/games101_with_rust
cd games101_with_rust
# 写作业1，修改 assignment1/main.rs，用你的代码替换 `todo!()` 宏
cargo r --bin assignment1 -r # 执行程序，-r 指 release mode
cargo r --bin assignment1 -- -r 0 output.png # 生成图像
```

### Linux

Linux 需要安装窗口库需要的依赖：

```shell
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```

## TODO

- [ ] 剩余作业
