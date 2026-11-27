# Command line

`pvcs` is the PolymerVCS command-line client. It's installed along with the GUI client on Windows and macOS, and in its own package on Linux. See [Install the GUI client](install.md).

`pvcs` uses the GUI client's sign-ins, so once you've signed in to a server in the GUI client, `pvcs` is signed in to it too.

Run the commands that work on a workspace from the workspace's top folder, the one with `.pvcs` in it. Running them from a folder inside it doesn't work.

> [!NOTE]
> `pvcs` only covers part of what the GUI client does so far. Use the GUI client for diffs, resolving merges, undoing changes, and deleting commits.

## Get started

```bash
pvcs signin http://vcs.example.com:3000 you@example.com 'your password'
pvcs create-workspace http://vcs.example.com:3000 my-game
cd my-game
```

## Commands

### `pvcs signin <server-url> <email> <password>`

Signs in to a server. The password is part of the command, so your shell may save it in its history. To avoid that, sign in with the GUI client instead.

### `pvcs signout`

Signs out of the current workspace's server on this computer, in the GUI client as well.

### `pvcs create-workspace <server-url> <name>`

Creates a workspace in a new folder called `<name>`, in the current folder, and fills it with the files from the latest commit on `main`.

### `pvcs status`

Shows which branch the workspace is on. It doesn't list pending changes yet, so ignore the **No changes detected** at the end, and use the GUI client's **Pending Changes** tab to see them.

### `pvcs refresh-pending`

Looks through the workspace for changes, and updates its pending changes. The GUI client does this as you work, but `pvcs` doesn't, so run this before you commit with `pvcs`.

### `pvcs commit <comment>`

Commits the checked pending changes, with `<comment>` as the commit's comment. Put the comment in quotes when it has spaces in it:

```bash
pvcs refresh-pending
pvcs commit "Add the forest level"
```

### `pvcs find branches`

Lists every branch, by its full name.

### `pvcs find commits`

Lists every commit, as its ID, then `/`, then its comment.

### `pvcs branch <name>`

Creates a branch called `<name>` from the workspace's commit. The workspace stays where it is.

### `pvcs switch <branch>`

Moves the workspace to the latest commit of a branch, given by its full name, such as `main/feature`.

> [!WARNING]
> Unlike the GUI client, `pvcs switch` doesn't rewrite your files to match the branch yet, or check for pending changes first. Switch branches in the GUI client.

### `pvcs merge-from-branch <branch>`

Starts merging from the latest commit of a branch, given by its full name. Resolve and apply the merge in the GUI client. See [Merging](merging.md).

### `pvcs update`

Checks whether there's a newer version of PolymerVCS.

### `pvcs --help` and `pvcs --version`

Lists the commands, or shows which version you have.

## Not available yet

`pvcs --help` also lists `lock` and `merge-resolve`, which don't work yet.
