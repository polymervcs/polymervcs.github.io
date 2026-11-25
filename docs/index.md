# PolymerVCS documentation

PolymerVCS is a version control system for game development, and for other projects that mix text with large binary files. A team shares one PolymerVCS server, and each person works in a workspace on their own computer.

> [!WARNING]
> PolymerVCS is in alpha. Expect bugs, feature changes, and irrecoverable data loss. Don't use it for production work yet, and keep a copy of anything you can't afford to lose.

## The parts of PolymerVCS

- **The PolymerVCS app** is where you do your work: commit changes, create and switch branches, and merge. It runs on Windows, macOS and Linux.
- **`pvcs`** is the command-line client. It does some of what the app does, from a terminal or a script.
- **PolymerVCS Server** holds the repository and the accounts of the people who use it. Its web page is where admins invite people and choose what each of them is allowed to do.

Everything is on the [downloads page](/downloads).

## Where to start

If someone has invited you to their server, [install the app](user/install.md), then [get started](user/get-started.md). [Concepts](user/concepts.md) explains how workspaces, branches and commits fit together.

If you're setting up a server for your team, [install the server](admin/install.md), then [set it up](admin/setup.md).

## Not available yet

These aren't in PolymerVCS yet:

- File locking. Admins can already set up [lock rules](admin/lock-rules.md), but no files are locked.
- Renaming and deleting branches in the app.
- Changing someone's role, or resetting a forgotten password. See [Users and invites](admin/users.md).
- HTTPS in the server itself. See [Set up the server](admin/setup.md#keep-traffic-private).
