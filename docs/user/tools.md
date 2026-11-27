# Diff and merge tools

PolymerVCS compares and merges files with tools you already have. Choose them in **Settings**: click the settings button at the top right of any screen.

- The **merge tool** is what **Merge in external tool** opens, to resolve a conflict. See [Merging](merging.md).
- The **diff tool** is what **Diff to workspace** and the other **Diff** items open, to compare two versions of a file.

Each can be **None**, **Beyond Compare**, **Meld**, or **Custom**.

## Beyond Compare and Meld

PolymerVCS runs Beyond Compare as `bcomp`, or as `bcompare` where there's no `bcomp`, and Meld as `meld`. The tool has to be on your `PATH`. On Windows, Beyond Compare's installer doesn't add its folder to `PATH`, so add it yourself, or use **Custom** with the full path to `BComp.exe`.

## Custom tools

For **Custom**, enter the command to run. It's split into arguments the way a shell would split it, and these placeholders are replaced with the paths of the files:

| Tool | Placeholder | Replaced with |
| --- | --- | --- |
| Merge | `$WORKSPACE` | Your version |
| Merge | `$INCOMING` | The version from the branch you're merging from |
| Merge | `$ANCESTOR` | The version both started from |
| Merge | `$MERGED` | Where to save the result |
| Diff | `$OLD` | The older version |
| Diff | `$NEW` | The newer version, or yours |

`$MERGED` starts out as a copy of your version. PolymerVCS waits for the merge tool to exit, and the conflict is only resolved if `$MERGED` was saved by then. So a tool that returns straight away and keeps running, like many editors, needs its wait option. The diff tool isn't waited for.

Some examples:

```text
meld "$WORKSPACE" "$ANCESTOR" "$INCOMING" --output "$MERGED"
code --wait --merge "$WORKSPACE" "$INCOMING" "$ANCESTOR" "$MERGED"
```

```text
meld "$OLD" "$NEW"
code --diff "$OLD" "$NEW"
```

## Check for updates

The top of **Settings** shows which version you have. Click **Check for update** to see whether there's a newer one. To update, see [Install the GUI client](install.md#updates).

## Where settings are kept

The GUI client's settings, your sign-ins and your list of recent workspaces are in `.pvcs/config.json` in your home folder (`%USERPROFILE%\.pvcs\config.json` on Windows). `pvcs` uses the same file. It holds what keeps you signed in, so don't share it.

If the file can't be read, the GUI client renames it to `config.json.bak` and starts again with default settings.
