# Commit changes

## See your changes

The **Pending Changes** tab lists how your workspace differs from its commit, in four groups:

| Group | Badge | What's in it |
| --- | --- | --- |
| Changed items | C | Files you've edited |
| Added items | A | New files and folders |
| Deleted items | D | Files and folders you've deleted |
| Renamed items | R | Files and folders you've renamed or moved |

While a workspace is open in the app, PolymerVCS watches its folder, and your changes appear as you save them. Changes made while the workspace wasn't open don't appear until you click the **Refresh** button, at the top of the tab.

Files that match a pattern in a `.pvcsignore` file aren't listed. See [Ignoring files](ignoring-files.md).

## Choose what to commit

Each item has a checkbox. The checked items are the ones the next commit includes, and the ones **Undo** reverts. New changes start out checked, so uncheck anything you want to leave out. The checkbox on a group's heading checks or unchecks everything in the group.

To select several items, click one, then Shift-click or Ctrl-click others (Cmd-click on macOS). Right-click the selection for what you can do with it.

## Commit

1. Describe your change in the **Comment** box.
2. Click **Commit**.

PolymerVCS uploads the checked changes, and records them as a new commit on your workspace's branch. Unchecked changes stay pending.

You can only commit when your workspace is on its branch's latest commit. If it's on an older one, [create a branch](branches.md#create-a-branch) from that commit, and commit there.

## Undo changes

**Undo** puts every checked item back the way it is in your workspace's commit. Edits are discarded, deleted files come back, and added files are deleted.

> [!CAUTION]
> Undo can't be undone. The edits it discards and the added files it deletes are gone for good, so uncheck anything you want to keep first.

## Compare a file with its committed version

Right-click a changed file and click **Diff to workspace**. Your diff tool opens with the committed version on one side and yours on the other. With several files selected, it opens once for each. Choose a diff tool in [Settings](tools.md) first. Folders can't be diffed.

## Look at a commit

In **Branch Explorer**, select a commit, then click **Details** to see what it changed. Right-click a file there to diff it with the commit before.

## Change a commit's comment

Right-click a commit in **Branch Explorer** or **Commits** and click **Change Message**. Enter the new comment, and click **Save**.

What you can change depends on your role. Usually you can change the comments of your own commits, and admins can change anyone's.

## Delete a commit

Right-click a commit in **Branch Explorer** or **Commits**, click **Delete commit**, then click **Delete**. A commit can only be deleted when:

- it's the latest commit on its branch,
- no branch has been created from it,
- it hasn't been merged into another branch, and isn't being merged from in your workspace.

If your workspace is on the commit you're deleting, commit or undo your pending changes first. The workspace then moves back to the commit before it.

> [!CAUTION]
> Deleting a commit can't be undone.
