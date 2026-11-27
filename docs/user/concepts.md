# Concepts

## Servers and repositories

A PolymerVCS server holds one repository: every file, branch and commit of one project. You reach it by its URL, such as `http://vcs.example.com:3000`. Whoever runs the server gives you the URL, and an invite.

## Workspaces

A workspace is a folder on your computer with the repository's files in it, as they are at one commit, plus whatever you've changed since. You can have as many workspaces as you like, including several of the same repository.

Each workspace has a `.pvcs` folder at its top, where PolymerVCS keeps what it knows about the workspace. That includes a copy of the repository's history, so it grows along with the repository. Don't change or delete anything in it. On Windows, it's hidden.

## Branches

Every repository starts with one branch, `main`. Each other branch is created from a commit on an existing branch, which makes it a child of that branch.

A branch's full name shows where it is in this tree. A branch called `feature`, created from a commit on `main`, has the full name `main/feature`, and a branch called `experiment` created from it is `main/feature/experiment`. Two children of the same branch can't have the same name.

## Commits

A commit records a set of changes to the repository's files, with a comment describing them. Each commit is on one branch, and the newest is the branch's latest commit.

New commits always go on top of a branch's latest commit. If your workspace is on an older commit, you can look at the files as they were, but to commit changes from there, you [create a branch](branches.md#create-a-branch) from it first.

## Your workspace's commit

A workspace is always on one commit, or on a branch that doesn't have any commits yet. The GUI client shows the name of its branch at the top of the window, as `branch @ server URL`. Switching moves the workspace to another branch or commit, and rewrites its files to match.

## Pending changes

Pending changes are how the files in your workspace differ from its commit: files you've changed, added, deleted or renamed. The GUI client notices them as you work. Each one has a checkbox, and the checked ones are what **Commit** and **Undo** act on. See [Commit changes](changes.md).

## Merging

Merging brings the changes from another commit into your workspace. For example, you merge a finished feature branch into `main`, or merge `main` into your own branch to catch up with your team. Changes that don't overlap are combined for you. Files that were changed on both sides are conflicts, and you choose how to resolve each one. Then you commit the result. See [Merging](merging.md).

## Large files

PolymerVCS splits files into chunks, and stores each distinct chunk only once. Files with the same content take up the space of one, and versions of a large file that differ in only a few places share most of their chunks.

## Locks

Some files, like textures, models and scenes, can't be merged, so when two people edit one at the same time, one of them loses their work. Admins can already set up lock rules for files like these, but locking itself isn't available yet. Until it is, agree with your team who's working on which files.
