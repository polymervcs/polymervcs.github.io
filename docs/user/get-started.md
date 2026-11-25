# Get started

To start, you need:

- the PolymerVCS app, [installed](install.md),
- your server's URL, such as `http://vcs.example.com:3000`,
- an invite to the server, from whoever runs it.

## Accept your invite

Your invite arrives either as an email titled "You have been invited to a PolymerVCS Server", or as a link your admin sends you some other way.

1. Open the link in a web browser.
2. Choose a password. It has to be strong enough to turn the strength bar green.
3. Click **Set password**.

That also signs you in to the server's web page. You don't need the web page to use PolymerVCS, so you can close it.

The link only works once, and it expires 7 days after it was sent. If it's expired, ask your admin to resend your invite.

## Sign in to the app

1. Open PolymerVCS.
2. Enter the server's URL. Include the `http://` or `https://`, and the port if there is one, such as `http://vcs.example.com:3000`.
3. Enter your email address and password, then click **Submit**.

The app keeps you signed in. If you haven't used it for about a day, it asks you to sign in again.

## Create a workspace

On the home screen:

1. Click **Create Workspace**.
2. Enter a **Workspace Name**. That's the name of the workspace's folder.
3. Click **...** and choose the **Parent Directory**, the folder to create the workspace in.
4. Click **Create Workspace**.

PolymerVCS creates the folder, downloads the repository into it, and fills it with the files from the latest commit on `main`. Then it opens the workspace. There can't already be a workspace in that folder.

## Open a workspace

The home screen lists your recent workspaces on this server. Click one to open it. To open a workspace that isn't listed, click **Open Workspace** and choose its folder.

The sign-in screen lists your recent workspaces too, with their servers. Click one to fill in its server's URL, and it opens once you've signed in.

## Find your way around

At the top of a workspace's window are:

- **Home**, which goes back to the home screen,
- your email address,
- the workspace's branch and server, as `branch @ server URL`,
- the settings button, at the right, which opens [Settings](tools.md).

Below them are tabs:

- **Pending Changes** lists what you've changed since your workspace's commit. See [Commit changes](changes.md).
- **Branch Explorer** draws the repository's branches and commits over time. See [Branches](branches.md).
- **Branches** lists every branch.
- **Commits** lists every commit.
- **Merge** appears while you're resolving a merge. See [Merging](merging.md).

Most actions are in the menu you get when you right-click a file, branch or commit.

## Sign out

On the home screen, click **Sign out**. The app forgets your sign-in on this computer. Your workspaces stay as they are.
