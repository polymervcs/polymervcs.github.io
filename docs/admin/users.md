# Users and invites

People join a server by invite. Manage them on the **Users** page of the server's web UI, which lists everyone's email address, role, when they were added, and whether they're **Active** or still **Invited**.

What you can do there depends on your role. See [Roles and permissions](roles.md).

## Invite someone

1. Under **Invite a user**, enter their email address.
2. Click **Send invite**.

Everyone you invite gets the **developer** role.

If [email is set up](configuration.md#email), the server emails them a link, and the page says the invite is on its way. Otherwise, or if the email couldn't be sent, the page shows the link instead. Copy it and send it to them yourself.

The link takes them to a page where they choose a password. It works once, and expires after 7 days. Once they've used it, they show as **Active**, and can sign in to the GUI client. Send them the server's URL too, and point them to [Get started](../user/get-started.md).

Each email address can only have one account on a server.

## Resend an invite

If someone's invite expired, or they lost it, click **Resend invite** in their row. They get a new link, and the old one stops working.

## Remove someone

Click **Remove user** in their row, then **Remove**. They're signed out everywhere straight away, and can't sign in again. The commits and branches they made stay. You can't remove your own account.

## Change someone's role

There's no way to change someone's role yet. The owner account you [created first](setup.md#create-the-admin-account) has the **admin** role, and everyone else has **developer**. To change what people can do, change what the developer role allows, which changes it for all of them. See [Roles and permissions](roles.md).

## Forgotten passwords

There's no password reset yet. Someone who has forgotten their password has to be removed and invited again. They come back as a new user, so they lose what their role only allows on their own branches and commits, such as changing their old commits' comments.

## Sign-ins

People stay signed in to the GUI client until they haven't used it for about a day. Signing out of the server's web UI ends that sign-in on the server too.

To sign everyone out at once, such as when you think a sign-in has been stolen, stop the server, delete `auth/auth.secret` in its [data folder](data.md), and start it again. The server makes a new one, and every sign-in ends.
