# PolymerVCS documentation

PolymerVCS is a version control system for game development, and for other projects that mix text with large binary files.

> [!WARNING]
> PolymerVCS is in alpha. Expect bugs, feature changes, and irrecoverable data loss. Don't use it for production work yet, and keep a copy of anything you can't afford to lose.

## PolymerVCS apps

- **GUI Client** allows users to commit changes, create and switch branches, and merge to a repository.
- **CMD Client** For those who don't want to use a GUI, and for scripting & CI/CD pipelines.
- **Server** holds the repository and user accounts. Its web UI allows admins to set up users, roles, and permissions.

Everything is on the [downloads page](/downloads), and every app works on Linux, macOS, and Windows.

## Where to start

If someone has invited you to their server, [install a client](user/install.md), then [get started](user/get-started.md). [Concepts](user/concepts.md) explains how workspaces, branches and commits fit together.

If you're setting up a server for your team, [install the server](admin/install.md), then [set it up](admin/setup.md).

## Current Limitations

- File locking. Admins can already set up [lock rules](admin/lock-rules.md), but no files are locked.
- Changing someone's role, or resetting a forgotten password. See [Users and invites](admin/users.md).
- HTTPS in the server itself. See [Set up the server](admin/setup.md#keep-traffic-private).
