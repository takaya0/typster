# ビルドタスク

[English version](../en/build-tasks.md)

Typster は Typst ファイルのコンパイルと作業のための6つの Zed タスクを提供します。

## 利用可能なタスク

| タスク | コマンド | 説明 |
|--------|---------|------|
| `typst compile` | `typst compile <ファイル>` | デフォルト出力形式にコンパイル |
| `typst watch` | `typst watch <ファイル>` | ウォッチモード — 保存のたびに自動再コンパイル |
| `typst compile (to PDF)` | `typst compile --format pdf <ファイル>` | 明示的に PDF にコンパイル |
| `typst compile (to SVG)` | `typst compile --format svg <ファイル>` | SVG にコンパイル |
| `typst compile (to PNG)` | `typst compile --format png <ファイル>` | PNG にコンパイル |
| `typst fonts` | `typst fonts --variants` | 利用可能な全フォントとバリアントを一覧表示 |

`<ファイル>` のプレースホルダーは、タスク実行時に現在開いているファイルのパスに置き換えられます。

## タスクの実行方法

**コマンドパレットから**:
1. **⌘⇧P** を押してコマンドパレットを開く
2. `task: spawn` と入力
3. リストからタスクを選択

**ランナブルボタンから**:
`.typ` ファイルにはエディタの余白に ▶（再生）ボタンが表示されます。クリックするとそのファイルのデフォルト `typst compile` タスクが実行されます。

## 前提条件

`typst` CLI がインストールされ、PATH に通っている必要があります。[typst.app](https://typst.app) またはパッケージマネージャーからインストールしてください：

```sh
# Homebrew（macOS）
brew install typst

# Cargo
cargo install --git https://github.com/typst/typst --locked typst-cli
```

インストールの確認：
```sh
typst --version
```
