# PDF プレビューと Forward Search

[English version](../en/pdf-preview.md)

Typster はシステムにインストールされているものに基づいて、2つの PDF プレビューモードを自動的に選択します。

---

## ブラウザプレビュー（デフォルト）

対応する外部ビューワーが見つからない場合、Typster は tinymist の組み込みブラウザプレビューサーバーを有効化します。

**要件**: tinymist 0.13.6 以降（キャッシュされていない場合は自動ダウンロード）。

**使い方**:
1. Zed で `.typ` ファイルを開く
2. ブラウザで `http://127.0.0.1:23635` にアクセス
3. 入力中にリアルタイムでプレビューが更新される
4. **Forward Search**: Zed でカーソルを移動すると、ブラウザが対応箇所に自動スクロール
5. **Inverse Search**: ブラウザ内をクリックすると、Zed が対応するソース行にジャンプ

> ブラウザプレビューには追加設定が不要です。Forward Search と Inverse Search は WebSocket 経由で自動動作します。

---

## 外部ビューワープレビュー

対応する PDF ビューワーがインストールされている場合、Typster は起動時に自動的に：
- ビューワーを検出する
- そのビューワー用の `forwardSearch.command` と `forwardSearch.args` を設定する
- `exportPdf: "onSave"` を設定してビューワーが開ける PDF を用意する

### ビューワー検出の優先順

ビューワーは以下の順で検出されます。最初に見つかったものが使用されます。

| 優先度 | ビューワー | プラットフォーム | 検出方法 |
|--------|----------|--------------|---------|
| 1 | Skim | macOS | `/Applications/Skim.app/Contents/SharedSupport/displayline` が存在する、または PATH に `skimapp` がある |
| 2 | SumatraPDF | Windows | PATH に `SumatraPDF` がある |
| 3 | Zathura | Linux / クロスプラットフォーム | PATH に `zathura` がある |
| 4 | Sioyek | クロスプラットフォーム | PATH に `sioyek` がある |
| 5 | Okular | Linux / KDE | PATH に `okular` がある |
| 6 | Evince | Linux / GNOME | PATH に `evince` がある |

### 自動設定される Forward Search 引数

Typster は以下の `forwardSearch` 引数を自動設定します：

| ビューワー | コマンド例 |
|----------|----------|
| Skim | `displayline %l %p %i` |
| SumatraPDF | `SumatraPDF -forward-search %i %l %p` |
| Zathura | `zathura --synctex-forward %l:1:%i %p` |
| Sioyek | `sioyek --reuse-window --execute-command toggle_synctex --forward-search-file %i --forward-search-line %l --open %p` |
| Okular | `okular --unique file:%p#src:%l%i` |
| Evince | `evince --forward-search %i %l %p` |

**プレースホルダーの意味**:
- `%l` — 現在の行番号
- `%p` — コンパイルされた PDF ファイルのパス
- `%i` — Typst ソースファイルのパス

---

## プレビューワーの選択

設定の `typsterPreviewer` で自動検出を上書きできます：

```jsonc
{
  "lsp": {
    "tinymist": {
      "settings": {
        "typsterPreviewer": "browser"
      }
    }
  }
}
```

| 値 | 動作 |
|----|------|
| `"auto"` | 優先順でインストール済みビューワーを検出、見つからない場合はブラウザにフォールバック（デフォルト） |
| `"browser"` | 外部ビューワーがインストールされていても、常にブラウザプレビューを使用 |
| `"skim"` | Skim を使用、インストールされていない場合はブラウザにフォールバック |
| `"sumatrapdf"` | SumatraPDF を使用、インストールされていない場合はブラウザにフォールバック |
| `"zathura"` | Zathura を使用、インストールされていない場合はブラウザにフォールバック |
| `"sioyek"` | Sioyek を使用、インストールされていない場合はブラウザにフォールバック |
| `"okular"` | Okular を使用、インストールされていない場合はブラウザにフォールバック |
| `"evince"` | Evince を使用、インストールされていない場合はブラウザにフォールバック |

---

## Inverse Search（PDF → ソース）

ブラウザプレビューの場合、Inverse Search は自動で動作します。

外部ビューワーの場合、PDF 内でクリックしたときに `zed` を呼び出すようビューワー側を設定する必要があります。各ビューワーのステップバイステップの手順は [inverse-search.md](inverse-search.md) を参照してください。

---

## プレビュー設定の上書き

Typster はユーザー設定を自動検出値にディープマージするため、他の設定を失わずに特定のキーだけを上書きできます。例えば、ブラウザプレビューを有効のまま更新モードだけを変更する場合：

```jsonc
{
  "lsp": {
    "tinymist": {
      "settings": {
        "preview": {
          "refresh": "onSave"
        }
      }
    }
  }
}
```

ディープマージの詳細な説明は [configuration.md](configuration.md) を参照してください。
