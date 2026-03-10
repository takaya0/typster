# Inverse Search（PDF → ソース）設定ガイド

## 概要

**Inverse Search** とは、PDF ビューワー上でクリックすると、対応する Typst ソースファイルの該当行に Zed エディタがジャンプする機能です。

### なぜビューワー側の設定が必要か

Forward Search（ソース → PDF）は Typster が tinymist LSP へ設定を渡すため自動で動作します。一方、Inverse Search はビューワーがエディタを呼び出す仕組みのため、**各ビューワーのアプリ設定でコマンドを登録する必要があります**。Typster はこの設定を自動化できません。

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
