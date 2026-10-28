# 機能一覧

[English version](../en/features.md)

## Language Server（tinymist）

Typster は [tinymist](https://github.com/Myriad-Dreamin/tinymist)（フル機能の Typst LSP サーバー）を統合しており、以下を提供します：

- **補完** — 関数名、パラメータ、シンボル、パッケージインポート
- **ホバードキュメント** — 関数・変数のインラインドキュメント表示
- **診断** — コンパイルエラーと警告をインライン表示
- **定義へジャンプ** — 関数・変数の定義箇所へ移動
- **参照を探す** — プロジェクト全体でのシンボル使用箇所を一覧表示
- **シンボルのリネーム** — 関数・変数・ラベルを安全にリネーム
- **コードアクション** — クイックフィックスとリファクタリング提案
- **シグネチャヘルプ** — 関数呼び出し入力中のパラメータヒント

tinymist の全機能については [tinymist のドキュメント](https://github.com/Myriad-Dreamin/tinymist) を参照してください。

## シンタックスハイライト

Tree-sitter ベースのグラマー（[uben0/tree-sitter-typst](https://github.com/uben0/tree-sitter-typst)）により以下を提供します：

- マークアップ・数式・コードモードの正確なハイライト
- 見出し・関数・ブロックでのコード折りたたみ
- スマートインデント
- 段落・関数選択のテキストオブジェクト
- ドキュメントアウトライン（Zed のアウトラインパネルに見出しを表示）
- ランナブル（`.typ` ファイルへのビルドタスクボタン）

**セマンティックトークン**はデフォルトで有効になっており、Tree-sitter レイヤーの上に LSP 駆動のハイライトを追加し、型・変数の色分けをより正確にします。不要な場合は `semanticTokens: "disable"` で無効化できます。

## コードラベル

Typster はリッチな補完・シンボルラベルを提供します。オートコンプリートポップアップの補完アイテムにシンタックスハイライト付きのシグネチャが表示されるため、ホバードキュメントを開かなくても関数・変数・シンボルを識別しやすくなります。

## PDF プレビュー

インストール済みのソフトウェアに基づいて、2つのプレビューモードが自動的に選択されます：

| モード | 有効になる条件 | URL / トリガー |
|--------|--------------|--------------|
| ブラウザプレビュー | 外部ビューワーが見つからない場合（デフォルト） | `http://127.0.0.1:23635` |
| 外部ビューワー | Skim / SumatraPDF / Zathura / Sioyek / Okular / Evince を検出 | PDF が自動的に開く |

セットアップの詳細とビューワー検出の優先順については [pdf-preview.md](pdf-preview.md) を参照してください。

## Forward Search

Zed のカーソル位置から PDF の対応箇所にジャンプします：

- **ブラウザプレビュー**: WebSocket 経由で自動 — カーソル移動でブラウザがスクロールします。
- **外部ビューワー**: ビューワー固有のショートカットでトリガー（Typster が自動設定）。

## Inverse Search（PDF → ソース）

PDF をクリックして Zed の対応するソース行にジャンプします：

- **ブラウザプレビュー**: ブラウザプレビュー内をクリックするだけ。
- **外部ビューワー**: ビューワー側での一度限りの手動設定が必要。[inverse-search.md](inverse-search.md) を参照。

## フォーマット

Typster は PATH 上のコードフォーマッターを検出し、tinymist を自動設定します：

| フォーマッター | 検出方法 | 優先度 |
|-------------|---------|-------|
| [typstyle](https://github.com/Enter-tainer/typstyle) | PATH 上に `typstyle` バイナリがある | 高い |
| [typstfmt](https://github.com/astrale-sharp/typstfmt) | PATH 上に `typstfmt` バイナリがある | 低い |

**⌥⇧F** または `editor: format` で現在のドキュメントをフォーマットします。設定の `formatterMode` で検出されたフォーマッターを上書きできます。

## ビルドタスク

Zed のタスクランナーから6つのタスクを実行できます：

| タスク | 説明 |
|--------|------|
| `typst compile` | デフォルト出力にコンパイル |
| `typst watch` | 保存時に継続的にコンパイル |
| `typst compile (to PDF)` | PDF 形式で出力 |
| `typst compile (to SVG)` | SVG 形式で出力 |
| `typst compile (to PNG)` | PNG 形式で出力 |
| `typst fonts` | 利用可能なフォントとバリアントを一覧表示 |

使い方の詳細は [build-tasks.md](build-tasks.md) を参照してください。

## スラッシュコマンド

Zed の AI アシスタントパネルで2つのスラッシュコマンドを使用できます：

| コマンド | 説明 |
|---------|------|
| `/typst-docs <関数名>` | Typst 標準ライブラリ関数のシグネチャ、パラメータ、使用例を表示 |
| `/typst-symbols <カテゴリ>` | カテゴリ別の数式シンボルを表示（arrow, greek, operator, relation, set, logic, accent, misc） |

使い方の例は [slash-commands.md](slash-commands.md) を参照してください。
