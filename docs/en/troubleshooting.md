# Troubleshooting

[日本語版](../ja/troubleshooting.md)

## tinymist is not starting

**Symptoms**: No completions, no diagnostics, status bar does not show "Typst language support".

1. Check Zed's logs: **Zed > View > Logs** (or run `zed: open log` from the command palette). Look for errors mentioning `tinymist`.
2. If you have a custom `binary.path` set, verify the path is correct and the binary is executable.
3. If you are on macOS Intel: Typster's auto-downloaded binary is arm64 only. Install tinymist manually and set `lsp.tinymist.binary.path`.
4. Try reloading the window: `zed: reload` from the command palette.

## PDF is not updating

**Symptoms**: The PDF viewer shows an old version of the document.

- In **external viewer mode**: The PDF is regenerated on save (`exportPdf: "onSave"`). Make sure you have saved the file. If the viewer does not reload automatically, check its auto-reload settings.
- In **browser preview mode**: The preview should update in real time. If it has stopped updating, reload the browser tab (`http://127.0.0.1:23635`).

## Browser preview is not loading

**Symptoms**: `http://127.0.0.1:23635` shows a connection error.

1. Check that tinymist is running (see "tinymist is not starting" above).
2. Verify that tinymist 0.13.6 or later is being used. Check the logs for the tinymist version.
3. Check if another process is using port 23635: `lsof -i :23635`
4. Try setting `typsterPreviewer: "browser"` explicitly to force browser mode.

## Formatter is not working

**Symptoms**: **Format Document** (⌥⇧F) does nothing or shows an error.

1. Check that `typstyle` or `typstfmt` is installed and on PATH:
   ```sh
   which typstyle   # or: which typstfmt
   ```
2. If the binary is installed but not detected, restart Zed after installation (Typster checks PATH at startup).
3. You can also set the formatter explicitly in settings:
   ```jsonc
   { "lsp": { "tinymist": { "settings": { "formatterMode": "typstyle" } } } }
   ```

## Forward search is not working

**Symptoms**: Moving the cursor in Zed does not scroll the PDF viewer.

- **Browser preview**: Forward search happens automatically. If it is not working, check that the browser tab is open at `http://127.0.0.1:23635` and reload it.
- **External viewer**: Check what viewer was detected by looking at the Zed logs. If the wrong viewer is detected (or no viewer), use `typsterPreviewer` to specify one explicitly.

## Inverse search is not working

See [inverse-search.md](inverse-search.md) for per-viewer setup instructions.

**Quick checklist**:
- Is the `zed` CLI on PATH? Run `which zed`.
- Has the editor command been saved in the viewer's settings?
- Is the `zed` CLI able to open the file? Try `zed /path/to/file.typ:10` from the terminal.

## Semantic tokens are not highlighting correctly

If token colours look wrong or the same as without Typster:

1. Check that `semanticTokens` is not set to `"disable"` in your settings.
2. The theme must support semantic token scopes. Try switching to a different Zed theme.

## How to check Zed extension logs

1. Open the command palette (**⌘⇧P**)
2. Run `zed: open log`
3. Look for lines containing `tinymist` or `typster`

Or from the menu: **Zed > View > Logs**.
