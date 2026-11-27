# Set up the server

## Create the admin account

As soon as the server is [installed](install.md), open its web page in a browser, at `http://<server>:3000`, such as `http://vcs.example.com:3000`. On the computer the server runs on, that's `http://localhost:3000`.

A new server opens on **Welcome to PolymerVCS**:

1. Enter your email address.
2. Choose a password. It has to be strong enough to turn the strength bar green.
3. Click **Create Owner**.

This first account has the admin role, which can do everything. Then sign in with it.

> [!IMPORTANT]
> Until this account exists, anyone who can reach the server can create it, and take the server over. Create it right after installing, before you open the server up to your network.

## The server's web page

Once you're signed in, the bar at the top has links to:

- **Users**, where you invite people and remove them. See [Users and invites](users.md).
- **Roles & Permissions**, where you choose what each role can do. See [Roles and permissions](roles.md).
- **Lock Rules**. See [Lock rules](lock-rules.md).

Everyone on the server can sign in to the web page, but the links only appear for people whose role can use them. The bar also shows who's signed in, the server's version, and when there's a newer version available.

## Next steps

1. Set `base_url` in the server's [configuration](configuration.md) to the address your team reaches the server at, such as `http://vcs.example.com`. Invite links are made from it, and until it's set, they point at `localhost`.
2. To have invites emailed, [set up email](configuration.md#email). Without it, you copy each invite link and send it yourself.
3. [Invite your team](users.md).
4. Send them the server's URL, such as `http://vcs.example.com:3000`, and point them to [Install the GUI client](../user/install.md) and [Get started](../user/get-started.md).

## Keep traffic private

The server only speaks plain HTTP. Everything sent to it, including passwords, crosses the network unencrypted, so only run it on a network you trust, such as an office network or a VPN.

To use HTTPS, put a reverse proxy that handles HTTPS, such as Caddy or nginx, in front of the server, and have people use its `https://` address. Invite links still use the server's own port, though. If the proxy is on another port, such as 443 while the server is on 3000, emailed invites would have the wrong link in them. Leave email off, and correct the port in each invite link before you send it.
