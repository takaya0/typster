# Inverse Search（PDF → ソース）設定ガイド

## 概要

**Inverse Search** とは、PDF ビューワー上でクリックすると、対応する Typst ソースファイルの該当行に Zed エディタがジャンプする機能です。

Typster は 2 種類のプレビュー方式に対応しています:

- **ブラウザプレビュー**（推奨・設定不要）: 外部ビューワーが検出されない場合、tinymist のバックグラウンドプレビューサーバーが自動的に有効化されます。ブラウザ（Chrome など）で `http://127.0.0.1:23635` を開くと SVG ベースのプレビューが表示され、Forward/Inverse Search は WebSocket 経由で自動的に動作します。
- **外部ビューワー**（手動設定が必要）: Skim, SumatraPDF, Zathura など PDF ビューワーをインストールすると自動検出されます。Forward Search は自動設定されますが、Inverse Search はビューワー側でエディタコマンドを登録する必要があります。

> **ブラウザプレビューの要件**: tinymist バージョン 0.13.6 以降が必要です。PATH 上にそれより古い tinymist がある場合はアップデートするか、設定で `lsp.tinymist.binary.path` に最新バイナリのパスを指定してください。

### プレビューワーの指定方法

Zed の `settings.json` で `typsterPreviewer` キーを指定することで、使用するプレビューワーを明示的に選択できます:

```json
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

指定可能な値:

| 値 | 説明 |
|----|------|
| `"auto"` | 自動検出（デフォルト）。外部ビューワーが見つからない場合はビルトインにフォールバック |
| `"browser"` | tinymist のブラウザプレビュー（Chrome など任意のブラウザで利用可）|
| `"skim"` | Skim（macOS）|
| `"sumatrapdf"` | SumatraPDF（Windows）|
| `"zathura"` | Zathura（Linux）|
| `"sioyek"` | Sioyek（クロスプラットフォーム）|
| `"okular"` | Okular（Linux / KDE）|
| `"evince"` | Evince（Linux / GNOME）|

> **注意**: 外部ビューワーを指定したがインストールされていない場合、自動的にブラウザプレビューにフォールバックします。

### ブラウザプレビューの使い方

1. `typsterPreviewer: "browser"` を設定するか、外部 PDF ビューワーをインストールしていない状態で Typst ファイルを開く
2. ブラウザ（Chrome など）で `http://127.0.0.1:23635` にアクセス
3. PDF をエクスポートせずリアルタイムプレビューが確認できます
4. ブラウザ上でクリックすると Zed の対応行にジャンプします（Inverse Search）
5. Zed でカーソルを動かすとブラウザのスクロール位置も追従します（Forward Search）

> **注意**: ユーザー設定で `preview` キーを部分的に上書きすると、自動設定の `preview` オブジェクト全体が置き換わります。これは既存の `forwardSearch` と同じ動作です。`background.enabled` を維持したまま他のキーを変更する場合は、`preview` オブジェクト全体を設定してください。

---

### なぜ外部ビューワーには手動設定が必要か

Inverse Search はビューワーがエディタを呼び出す仕組みのため、**各ビューワーのアプリ設定でコマンドを登録する必要があります**。Typster はこの設定を自動化できません。

## 前提条件

`zed` CLI が PATH に通っていること。Zed アプリのメニューから **Zed > Install CLI** を実行してください。確認コマンド:

```sh
which zed
# → /usr/local/bin/zed など
```

---

## ビューワー別セットアップ手順

### Skim（macOS）

1. Skim を開き、**Skim > Preferences** を開く
2. **Sync** タブを選択
3. **PDF-TeX Sync Support** セクションの Preset を **Custom** に設定
4. 以下を入力:
   - **Command**: `zed`
   - **Arguments**: `%file:%line`
5. Preferences を閉じる

PDF 上で `⌘` + クリックすると Zed が該当行を開きます。

---

### SumatraPDF（Windows）

1. SumatraPDF を開き、**Settings > Options** を開く
2. **Set inverse search command line** フィールドに以下を入力:

   ```
   zed "%f:%l"
   ```

3. **OK** をクリック

PDF 上でダブルクリックすると Zed が該当行を開きます。

---

### Zathura（Linux）

`~/.config/zathura/zathurarc` に以下を追加:

```
set synctex-editor-command "zed \"%{input}:%{line}\""
```

設定後 Zathura を再起動してください。PDF 上で `Ctrl` + クリックすると Zed が該当行を開きます。

---

### Sioyek（クロスプラットフォーム）

Sioyek のユーザー設定ファイル（`prefs_user.config`）に以下を追加:

```
inverse_search_command  zed "%1:%2"
```

ファイルの場所:
- **Windows**: `%APPDATA%\sioyek\prefs_user.config`
- **macOS**: `~/Library/Application Support/sioyek/prefs_user.config`
- **Linux**: `~/.config/sioyek/prefs_user.config`

設定後 Sioyek を再起動してください。

---

### Okular（Linux / KDE）

1. Okular を開き、**Settings > Configure Okular** を開く
2. **Editor** タブを選択
3. **Editor** ドロップダウンで **Custom Text Editor** を選択
4. **Command** フィールドに以下を入力:

   ```
   zed %f:%l
   ```

5. **OK** をクリック

PDF 上でクリックすると Zed が該当行を開きます。

---

### Evince（Linux / GNOME）

Evince は D-Bus 経由で Inverse Search を実装しており、エディタコマンドを直接設定する仕組みがありません。Evince での Inverse Search には対応していません。代わりに Zathura や Okular の使用を推奨します。

---

## トラブルシューティング

### クリックしても Zed が起動しない

- `zed` CLI が PATH に通っているか確認: `which zed`
- ビューワーの設定が保存されているか再確認

### 複数の Zed ウィンドウが開いている場合

`zed path:line` は既存のウィンドウで該当ファイルを開きます。どのウィンドウが前面に来るかはウィンドウマネージャーの挙動に依存します。

### ファイルパスにスペースが含まれる場合

ビューワーによってはコマンドをクォートする必要があります（SumatraPDF の `"%f:%l"` など）。各ビューワーの設定例にはクォートが含まれているため、そのままコピーしてください。

### Skim で行番号がずれる

Typst ファイルを保存してから PDF を再生成することで SyncTeX 情報が更新されます。ビルド後にもう一度試してください。
