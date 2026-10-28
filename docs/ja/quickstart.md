# クイックスタート

[English version](../en/quickstart.md)

3ステップで Typster を使い始められます。

## 前提条件

- [Zed](https://zed.dev) エディタがインストール済みであること
- [Typst](https://typst.app) 0.14.2 以降（`typst` CLI とビルドタスクに必要）
- tinymist は **自動ダウンロード** されます — 手動インストール不要

## Step 1: Typster のインストール

1. Zed を開く
2. **⇧⌘X** を押すか、コマンドパレットで `zed: extensions` を実行
3. **Typster** を検索
4. **Install** をクリック

## Step 2: Typst ファイルを開く

`.typ` 拡張子のファイルを作成または開きます。Zed が自動的に Typster を有効化します。

Zed のステータスバーに **"Typst language support"** と表示されれば、tinymist が起動したことを示します。補完、ホバードキュメント、診断が使用可能になります。

> **初回起動時**: 最初のファイルオープン時に、Typster が GitHub から最新の `tinymist` バイナリをダウンロードします。数秒かかります。2回目以降はキャッシュされたバイナリを使用します。

## Step 3: ドキュメントをプレビューする

Typster はシステムにインストールされているものに基づいて、PDF プレビューを自動設定します。

**外部 PDF ビューワーがインストールされていない場合（デフォルト）**

Typster は tinymist のブラウザ内プレビューを有効化します。ブラウザで以下の URL を開いてください：

```
http://127.0.0.1:23635
```

入力中にリアルタイムでプレビューが更新されます。ブラウザ内でクリックすると Zed の対応行にジャンプし（Inverse Search）、Zed でカーソルを移動するとブラウザのスクロール位置も追従します（Forward Search）。

**外部 PDF ビューワーがインストールされている場合**

Skim（macOS）、SumatraPDF（Windows）、Zathura、Sioyek、Okular、Evince がインストールされていると、Typster が自動的に検出して以下を設定します：
- 保存のたびに PDF をエクスポート（`exportPdf: "onSave"`）
- 現在のカーソル位置に対応する PDF 箇所を開く Forward Search の設定

検出優先順と設定オプションの詳細は [pdf-preview.md](pdf-preview.md) を参照してください。

## 舞台裏で何が起きているか

**tinymist バイナリの解決順序**：
1. Zed 設定の `lsp.tinymist.binary.path`
2. システム PATH 上の `tinymist`
3. 以前にキャッシュされたダウンロード済みバイナリ
4. GitHub（`Myriad-Dreamin/tinymist`）から最新リリースをダウンロード

**フォーマッターの自動検出**: PATH 上に `typstyle` または `typstfmt` が見つかった場合、Typster は `formatterMode` を自動設定します（`typstyle` が優先）。**Format Document**（⌥⇧F）でフォーマットを使用するには、いずれかのツールをインストールしてください。

## 次のステップ

- [features.md](features.md) — Typster が提供する全機能の一覧
- [configuration.md](configuration.md) — LSP、プレビュー、フォーマッターの設定カスタマイズ
- [slash-commands.md](slash-commands.md) — AI アシスタントで `/typst-docs` と `/typst-symbols` を使う
- [build-tasks.md](build-tasks.md) — コンパイルとウォッチタスク
