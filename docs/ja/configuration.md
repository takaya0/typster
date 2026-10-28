# 設定リファレンス

[English version](../en/configuration.md)

Typster の全設定は Zed の `settings.json` で行います。コマンドパレットで `zed: open settings` を実行して開いてください。

設定はトップレベル（グローバル）に記述するか、プロジェクトルートの `.zed/settings.json`（プロジェクトスコープ）に記述できます。

---

## tinymist バイナリ

使用する `tinymist` バイナリを指定します。Typster は以下の優先順でバイナリを解決します：

1. `lsp.tinymist.binary.path`（設定されている場合）
2. システム PATH 上の `tinymist`
3. 以前にキャッシュされたダウンロード済みバイナリ
4. `Myriad-Dreamin/tinymist` の最新 GitHub リリース（自動ダウンロード）

```jsonc
{
  "lsp": {
    "tinymist": {
      "binary": {
        // 特定の tinymist バイナリへの絶対パス。
        // バージョンを固定したい場合やカスタムビルドを使う場合に指定します。
        "path": "/usr/local/bin/tinymist",

        // tinymist 起動時に渡す追加引数。
        "arguments": []
      }
    }
  }
}
```

> **macOS 注意**: Typster は Apple Silicon（arm64）の macOS のみをサポートしています。Intel Mac ユーザーは tinymist を手動インストールし、`binary.path` を設定してください。

---

## Typster 固有の設定

`typsterPreviewer` は Typster が独自に持つ唯一のキーです。**Typster が消費し、tinymist には渡されません**。

```jsonc
{
  "lsp": {
    "tinymist": {
      "settings": {
        "typsterPreviewer": "auto"
      }
    }
  }
}
```

| 値 | 説明 |
|----|------|
| `"auto"` | インストール済みのビューワーを自動検出（デフォルト）。見つからない場合はブラウザプレビューにフォールバック |
| `"browser"` | インストール済みビューワーに関わらず、tinymist のブラウザプレビューを強制使用 |
| `"skim"` | Skim（macOS） |
| `"sumatrapdf"` | SumatraPDF（Windows） |
| `"zathura"` | Zathura（Linux） |
| `"sioyek"` | Sioyek（クロスプラットフォーム） |
| `"okular"` | Okular（Linux / KDE） |
| `"evince"` | Evince（Linux / GNOME） |

> 指定した外部ビューワーがインストールされていない場合、Typster は自動的にブラウザプレビューにフォールバックします。

---

## 自動検出される tinymist 設定

Typster は環境に基づいて以下の tinymist キーを自動設定します。`settings` に明示的な値を指定することで上書きできます。

| キー | 自動設定値 | 条件 |
|------|-----------|------|
| `exportPdf` | `"onSave"` | 外部ビューワーを検出した場合 |
| `exportPdf` | `"never"` | ブラウザプレビューモードの場合 |
| `formatterMode` | `"typstyle"` | PATH に `typstyle` がある場合 |
| `formatterMode` | `"typstfmt"` | PATH に `typstfmt` があり `typstyle` がない場合 |
| `semanticTokens` | `"enable"` | 常時（デフォルト） |
| `preview.background.enabled` | `true` | ブラウザプレビューモードの場合 |
| `preview.refresh` | `"onType"` | ブラウザプレビューモードの場合 |
| `forwardSearch.command` | ビューワーの実行ファイル | 外部ビューワーを検出した場合 |
| `forwardSearch.args` | ビューワー固有の引数 | 外部ビューワーを検出した場合 |

tinymist 設定の全一覧は [tinymist 設定リファレンス](https://github.com/Myriad-Dreamin/tinymist) を参照してください。

---

## ディープマージの動作

ユーザー設定は自動検出設定に**ディープマージ**されます — 自動検出された設定全体を置き換えるのではなく、再帰的に上書きします。

**例**: Typster がブラウザプレビューを自動検出し、以下を設定したとします：

```json
{
  "preview": {
    "background": { "enabled": true },
    "refresh": "onType"
  }
}
```

`refresh` だけを `"onSave"` に変更したい場合、そのキーだけ記述できます：

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

結果はマージ — `preview.background.enabled` は `true` のまま保持され、`refresh` だけが上書きされます。これは [issue #15](https://github.com/yataka/typster/issues/15) の修正による期待される動作です。

オブジェクト以外の値（文字列、数値、配列）については、ユーザー設定が常に優先されます。

---

## 初期化オプション

`initialization_options` は起動時に tinymist へ送信されます（ランタイムの `settings` とは別）。

```jsonc
{
  "lsp": {
    "tinymist": {
      "initialization_options": {
        // tinymist の初期化オプションをここに記述
      }
    }
  }
}
```

利用可能な初期化オプションについては [tinymist のドキュメント](https://github.com/Myriad-Dreamin/tinymist) を参照してください。

---

## 完全な設定例

よく使うカスタマイズを含む `settings.json` の例：

```jsonc
{
  "lsp": {
    "tinymist": {
      "binary": {
        // 任意: 特定のバイナリに固定
        "path": "/usr/local/bin/tinymist"
      },
      "settings": {
        // ブラウザプレビューを強制（外部ビューワーの自動検出をスキップ）
        "typsterPreviewer": "browser",

        // フォーマッターを明示指定（デフォルトは自動検出）
        "formatterMode": "typstyle",

        // プレビューの更新タイミングを変更
        "preview": {
          "refresh": "onSave"
        },

        // PDF エクスポートのタイミングを変更
        "exportPdf": "onType"
      }
    }
  }
}
```
