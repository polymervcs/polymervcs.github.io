# Branches

## See branches and commits

- **Branch Explorer** draws the repository's branches and their commits, laid out by date, with arrows from where each branch was created and from each merge. Your workspace's position is shown too. Select a commit and click **Details** to see what it changed.
- **Branches** lists every branch, by its full name.
- **Commits** lists every commit.

## Create a branch

1. Right-click the commit to start the branch from, in **Branch Explorer** or **Commits**, and click **Create branch from this commit**.
2. Enter a **Branch Name**.
3. To move your workspace onto the new branch, check **Switch to new branch**. You can't while you have pending changes.
4. Click **Create Branch**.

The new branch is a child of the commit's branch. Creating `feature` from a commit on `main` makes `main/feature`, for example. No two children of a branch can have the same name.

From the command line, `pvcs branch <name>` creates a branch from your workspace's commit. See [Command line](command-line.md).

## Switch branches

Right-click a branch in **Branches** and click **Switch to branch**. Your workspace moves to the branch's latest commit, and PolymerVCS rewrites its files to match. Files that a `.pvcsignore` ignores are left as they are.

To move to a particular commit instead, right-click it in **Branch Explorer** or **Commits** and click **Switch to commit**.

You can't switch while you have pending changes, or while a merge is in progress. Commit or undo your changes first.

New commits always go on a branch's latest commit, so switching to an older commit is for looking at the files as they were. To make changes from there, create a branch from it.

## Rename and delete branches

Renaming and deleting branches aren't available in the app yet. The **Rename Branch** item in a branch's menu doesn't do anything.
