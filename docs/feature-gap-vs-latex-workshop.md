# Feature Gap: typster vs LaTeX-Workshop

LaTeX-Workshop（VS Code用LaTeX拡張）と比較した、typsterの不足機能一覧。

参照: [James-Yu/LaTeX-Workshop](https://github.com/James-Yu/LaTeX-Workshop)

---

## typsterが既に持つ機能

| 機能 | 実装方法 |
|------|---------|
| LSP統合（補完、診断、ホバー、定義ジャンプ等） | tinymist |
| シンタックスハイライト | tree-sitter |
| コードフォーマット | typstyle / typstfmt 自動検出 |
| PDFプレビュー / Forward Search | 6ビューワー対応（Skim, SumatraPDF, Zathura, Sioyek, Okular, Evince） |
| PDF Export on Save | tinymist設定 |
| ビルドタスク | compile / watch / compile to PDF |
| コード折りたたみ | tree-sitter |
| 自動インデント | tree-sitter |
| ブラケットマッチング・自動クローズ | `()`, `[]`, `{}`, `""`, `$$` |
| ドキュメントアウトライン | 見出し6レベル |
| 言語インジェクション | rawブロック内の構文ハイライト |
| テキストオブジェクト | Vim用（function, class, comment） |

---

## 不足機能

### 高優先度

#### 1. Inverse Search（PDF → ソース）

- **LaTeX-Workshop**: PDF上でCtrl+Clickするとソースの該当行にジャンプ
- **typster**: Forward Search（ソース→PDF）のみ。Inverse Searchは未実装
- **備考**: tinymistがプロトコルレベルでサポートしている可能性があり、要調査

#### 2. ビルトインPDFビューワー（エディタ内タブ）

- **LaTeX-Workshop**: PDF.jsベースのビューワーをVS Codeのエディタタブ内に埋め込み、SyncTeX完全対応
- **typster**: 外部ビューワーのみ
- **備考**: Zed Extension APIの制約による可能性が高い

#### 3. ワードカウント

- **LaTeX-Workshop**: TeXCountユーティリティで文書の単語数をカウント
- **typster**: 未対応

---

### 中優先度

#### 4. 数式プレビューパネル

- **LaTeX-Workshop**: エディタ横にリアルタイム数式レンダリング専用パネルを表示（Ctrl+Alt+M）
- **typster**: 未対応
- **備考**: tinymist LSPがhover上で数式レンダリングを提供している可能性あり

#### 5. 画像プレビュー（ホバー）

- **LaTeX-Workshop**: 画像パス（`#image("...")` 相当）上でホバーするとサムネイルを表示
- **typster**: 未対応

#### 6. スニペット

- **LaTeX-Workshop**: 頻出パターン用の豊富なスニペット（環境、セクション、数式など）
- **typster**: 未対応（tinymist LSPが提供するスニペットに依存）
- **備考**: `languages/typst/snippets.json`等を追加することで対応可能

#### 7. セクション昇降（Promote / Demote）

- **LaTeX-Workshop**: セクションレベルの昇降コマンド（`=` ↔ `==` ↔ `===` 等）
- **typster**: 未対応

#### 8. 補助ファイルのクリーンアップ

- **LaTeX-Workshop**: ビルド時に生成される補助ファイル（`.aux`, `.log`等）を一括削除
- **typster**: 未対応
- **備考**: Typstはビルド補助ファイルをほぼ生成しないため優先度は低い

---

### 低優先度（Typstの設計上必要性が低い）

| 機能 | LaTeX-Workshop | typsterでの必要性が低い理由 |
|------|---------------|--------------------------|
| レシピベースのビルドシステム | latexmk → bibtex → latexmk 等の連鎖 | Typstは単一パスコンパイル |
| マジックコメント | `% !TEX root`, `% !TEX program` 等 | Typstは`#import`で明示的に指定 |
| BibTeX / BibLaTeX統合 | `.bib`ファイルの解析・補完 | Typstは独自形式（Hayagriva YAML）を使用 |
| リンティング | ChkTeX, LaCheck | tinymist LSPの診断で代替 |
| 環境操作コマンド | ラップ、名前変更、閉じる | Typstの構文はLaTeXほど環境依存でない |
| テキストフォーマットショートカット | `\textbf{}` 等の挿入 | Typstは `*bold*` 等のマークアップ構文 |
| パッケージドキュメントアクセス | texdocコマンド | Typstのパッケージエコシステムが異なる |

---

## まとめ

最も実装価値の高い不足機能:

1. **Inverse Search** - 実用性が高く、tinymistがサポートしている可能性あり
2. **スニペット** - `languages/typst/snippets.json`で比較的容易に追加可能
3. **数式プレビューパネル** - Typstユーザーにとって有用

ビルトインPDFビューワーはZed APIの制約により実装が困難な可能性がある。
