# Concepts

## Servers and repositories

Each PolymerVCS server holds one, and only one, repository: every file, branch, commit and the entire history of the project. You will be sent the invite link by an Admin, either directly or via email.

## Workspaces

A workspace is a folder on your computer with the repository's files in it, as they are at one commit, plus whatever you've changed since. You can have as many workspaces as you like, including several of the same repository.

Each workspace contains a hidden `.pvcs` folder. Here, PolymerVCS keeps all workspace info and a local copy of the repository's database. **Do Not** modify anything in this folder.

## Branches

Every repository starts with one branch, `main`. Each other branch is created from a commit on an existing branch, which makes it a child of that branch.

A branch's full name shows where it is in this tree. A branch called `feature`, created from a commit on `main`, has the full name `main/feature`, and a branch called `experiment` created from it is `main/feature/experiment`. Two children of the same branch can't have the same name.

## Commits

A commit records a set of changes to the repository's files, with a comment describing them. Each commit is on one branch, and the newest is the branch's latest commit.

New commits always go on top of a branch's latest commit. If your workspace is on an older commit, you can look at the files as they were, but to commit changes from there, you [create a branch](branches.md#create-a-branch) from it first.

## Your workspace's commit

A workspace is always either working off of a given commit, or an empty branch. The GUI client shows the name of its branch at the top of the window, as `branch @ server URL`. Switching moves the workspace to another branch or commit, and rewrites its files to match.

## Pending changes

Pending changes are how the files in your workspace differ from its commit: files you've changed, added, deleted or renamed. The GUI client notices them as you work. Each one has a checkbox, and the checked ones are what **Commit** and **Undo** act on. See [Commit changes](committing-changes.md).

## Merging

Merging brings the changes from another commit into your workspace. For example, you merge a finished feature branch into `main`, or merge `main` into your own branch to catch up with your team. Changes that don't overlap are combined for you. Files that were changed on both sides are conflicts, and you choose how to resolve each one. Then you commit the result. See [Merging](merging.md).
