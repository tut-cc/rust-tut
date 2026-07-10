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

#### NixOS-WSL

1. WSLのインストール\
   Powershell を管理者権限で開き、`wsl --install`
   を実行し、コンピュータを再起動する。参考:
   https://learn.microsoft.com/ja-jp/windows/wsl/install
2. NixOS-WSLのインストール\
   https://github.com/nix-community/NixOS-WSL/releases/tag/2605.7.2 の
   [nixos.wsl](https://github.com/nix-community/NixOS-WSL/releases/download/2605.7.2/nixos.wsl)
   をダウンロードして開く。
3. `wsl -d NixOS` でNixOS環境に入る。
4. 以下のコマンドを実行する。
   ```sh
   mkdir -p ~/.config/nix
   echo 'experimental-features = nix-command flakes' > ~/.config/nix/nix.conf
   sudo systemctl restart nix-daemon
   ```
5. `nix run nixpkgs#git clone https://github.com/tut-cc/rust-tut && cd rust-tut && nix develop`
   で開発シェルを起動する。

### Ubuntu

まずは https://rust-lang.org/ja/learn/get-started/ を参考に
rustupをインストールしてください。

```sh
sudo apt install -y build-essential curl
sudo apt install -y libwayland-client0 libxcursor1 libxi6 libxkbcommon-x11-0 # GUIアプリのライブラリ
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install stable
rustup component add rust-analyzer rust-src clippy rustfmt
```

その後、必要なツールをインストールします。

```sh
git clone https://github.com/tut-cc/rust-tut && cd rust-tut
cargo install rust-script
```
