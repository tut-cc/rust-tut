# Rust言語のチュートリアル

### [Lesson 1: 基本の文法 (詳細)](./l1_basics/README.md)

Rustの基本を学ぶ。

### [Lesson 2: 並行処理 (詳細)](./l2_concurrency/README.md)

GUIアプリケーションを作る。

## 実行環境の用意

### Nix

```sh
git clone https://github.com/tut-cc/rust-tut && cd rust-tut
nix develop
```

### NixOS-WSL

1. WSLのインストール\
   Powershell を管理者権限で開き、`wsl --install`
   を実行し、コンピュータを再起動する。参考:
   https://learn.microsoft.com/ja-jp/windows/wsl/install
2. NixOS-WSLのインストール\
   https://github.com/nix-community/NixOS-WSL/releases/tag/2605.7.2 の
   [nixos.wsl](https://github.com/nix-community/NixOS-WSL/releases/download/2605.7.2/nixos.wsl)
   をダウンロードして開く。
3. `wsl -d NixOS` でNixOS環境に入る。
4. `git clone https://github.com/tut-cc/rust-tut && cd rust-tut && nix develop`
   で開発シェルを起動する。

### 手動インストール

まずは https://rust-lang.org/ja/learn/get-started/ を参考に
rustupをインストールしてください。

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install stable
rustup component add rust-analyzer rust-src clippy rustfmt
```

その後、必要なツールをインストールします。

```sh
git clone https://github.com/tut-cc/rust-tut && cd rust-tut
cargo install rust-script
```
