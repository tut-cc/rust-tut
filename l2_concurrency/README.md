# 並列処理 (準備中)

マンデルブロ集合のプロットプログラムを用い、並列処理による高速化を試みます。

iced.rsというGUIフレームワークを用います。

## 実行

```sh
cargo run
```

Linuxで Wayland の問題が発生する場合、X11 に強制フォールバック:

```sh
WAYLAND_DISPLAY= WAYLAND_SOCKET= cargo run
```
