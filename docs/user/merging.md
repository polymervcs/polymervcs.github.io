# Merging

Merging brings the changes from another commit into your workspace's branch. You might merge a finished feature branch into `main`, or merge `main` into your own branch to catch up with your team.

Changes are merged *into* your workspace, so first switch to the branch that should get them, at its latest commit.

## Start a merge

1. Commit or undo your pending changes, so the merge can't overwrite them.
2. [Switch](branches.md#switch-branches) to the branch you're merging into.
3. Right-click the commit to merge from, usually the latest commit of the other branch, in **Branch Explorer** or **Commits**, and click **Merge from this commit**.

PolymerVCS compares both sides with the last commit they have in common, and opens the **Merge** tab. A workspace can only have one merge in progress at a time.

## Resolve conflicts

The top of the **Merge** tab says which branch you're merging from, and how many of the conflicts are resolved. Below that are:

- **Conflicting items**, which were changed on both sides. You resolve each of these.
- **Changed items**, **Added items** and **Deleted items**, which were only changed on the other side. They're merged for you when you apply the merge.

To resolve conflicts, select them, right-click, and choose one of:

- **Keep incoming changes**, to use the version from the branch you're merging from.
- **Keep workspace changes**, to keep your branch's version.
- **Merge in external tool**, to combine the two in your merge tool. It opens with your version, the incoming version, and the version both started from. The conflict is resolved once you save the merged file and close the tool. If you close it without saving, the conflict stays unresolved. Choose a merge tool in [Settings](tools.md) first.

A resolved conflict is written to your workspace straight away, and listed in the **Pending Changes** tab under **Merged changed items**, **Merged added items** or **Merged deleted items**. To change your mind about one, right-click it there and click **Undo resolution**.

## Apply and commit the merge

Once every conflict is resolved, click **Apply Merge**. PolymerVCS writes the rest of the other side's changes into your workspace, and the **Merge** tab closes. If applying the merge would overwrite one of your pending changes, it stops and says which, so you can undo that first.

The merged changes are now pending changes. Check that everything works, then [commit](changes.md#commit) as usual. A merge commit has to include every pending change, so leave them all checked. In **Branch Explorer**, an arrow shows where the merge came from.

## Cancel a merge

Click the **Cancel merge** button next to the name of the branch you're merging from. Each file the merge wrote goes back to how it was before the merge. If you've changed one of them since, undo that change first.

## From the command line

`pvcs merge-from-branch <branch>` starts a merge from the latest commit of a branch, given by its full name. Resolve and apply it in the app.
