# Troubleshooting

## Signing in

**Sign in Failed. Please check your username and password.**\
Check the server's URL as well as your email address and password. The URL needs its `http://` or `https://`, and its port if it has one, such as `http://vcs.example.com:3000`. If you haven't [accepted your invite](get-started.md#accept-your-invite) yet, do that first. If it still fails, check with your admin that the server is running and that your computer can reach it.

**Your session for … has expired. Please sign in again.**\
You haven't used PolymerVCS for a while. Sign in again in the GUI client.

**You are not signed in to …**\
`pvcs` has no sign-in for that server. Sign in with the GUI client, or with `pvcs signin`.

## Pending changes

**A change I made isn't listed.**\
Changes made while the workspace wasn't open in the GUI client, or with `pvcs`, only appear once you click **Refresh** in **Pending Changes**. If it's still missing, check whether a [`.pvcsignore`](ignoring-files.md) pattern matches it.

**None of the pending changes are marked to be committed**\
Check at least one item in **Pending Changes** before you commit.

**A commit on this branch already has this parent. Commit from the branch's latest commit or create a new branch**\
Your workspace is on an older commit of its branch, and new commits only go on a branch's latest commit. Create a branch from your workspace's commit and commit there. To switch to the new branch, you need to have no pending changes, so if you've already made changes, copy the changed files somewhere safe, undo them, create and switch to the branch, then copy them back.

## Switching

**Cannot switch while the workspace has pending changes**\
Commit or undo your pending changes, then switch.

**Cannot switch while a merge is in progress**\
Finish the merge by applying and committing it, or cancel it.

## Merging

**Branch does not exist**\
You can't merge into a branch that doesn't have any commits yet. Commit something on it first.

**Merge already in progress**\
A workspace has one merge at a time. Finish or cancel the current one first.

**All conflicts must be resolved before applying a merge**\
Resolve every item under **Conflicting items** in the **Merge** tab.

**Applying the merge would overwrite the pending changes at …, so undo them first**\
The merge would replace a file you've changed. Undo your change to it. Or, to keep the change, cancel the merge, commit the change, and start the merge again.

**A merge can only be committed when all pending changes are marked to be committed**\
A merge commit includes every pending change. Check them all, then commit.

**The merge tool closed without saving**\
The conflict is only resolved once you save the merged file in your merge tool. Try again, and save before closing it.

## Diff and merge tools

**No diff tool is set. Choose one in Settings.**\
Choose a diff tool in [Settings](tools.md).

**Couldn't run Beyond Compare (bcomp or bcompare)**, **Couldn't run Meld (meld)**\
The tool isn't installed, or isn't on your `PATH`. See [Diff and merge tools](tools.md#beyond-compare-and-meld).

## Permissions

**You do not have permission to do that**\
Your role on the server doesn't allow it. Ask your admin.

## Workspaces

**A version control repo already seems to exist at …**\
There's already a workspace in that folder. Open it with **Open Workspace**, or choose another name or parent folder.

**A workspace is missing from my recent workspaces.**\
The GUI client drops workspaces it can't find, such as ones that were moved or deleted. To add one back, open it with **Open Workspace**.
