# Roles and permissions

Each person on a server has one role, and a role is a set of permissions. Manage them on the **Roles & Permissions** page of the server's web page.

## The built-in roles

- **admin** has every permission, always. It can't be changed, renamed or deleted. The owner account you [created first](setup.md#create-the-admin-account) has it.
- **developer** is the role everyone you [invite](users.md) gets. It starts out with the permissions marked below, and you can change them.

You can add more roles, but they can't be given to anyone yet. See [Change someone's role](users.md#change-someones-role).

## Change what a role can do

1. Under **Permissions**, choose the role.
2. Check the permissions it should have, and uncheck the others.
3. Click **Save changes**, or **Discard** to go back.

The change applies to everyone with the role the moment you save.

To add a role, enter its name under **Add a role** and click **Add role**. To rename or delete one, use the buttons in its row. A role can only be deleted once nobody has it.

## Permissions

| Permission | What it allows | Developer |
| --- | --- | --- |
| List users | See the **Users** page | ✓ |
| Invite users | Invite people, and resend invites | |
| Remove users | Remove people from the server | |
| List permissions | See which permissions each role and person has | ✓ |
| List roles | See the **Roles & Permissions** page | ✓ |
| Edit roles | Add, rename and delete roles, and change their permissions | |
| List branches | See branches. Not checked yet | ✓ |
| Create branches | Create branches | ✓ |
| Edit branches | Rename any branch | |
| Edit own branches | Rename the branches you created | ✓ |
| Delete branches | Delete any branch | |
| Delete own branches | Delete the branches you created | ✓ |
| List commits | See commits. Not checked yet | ✓ |
| Create commits | Commit | ✓ |
| Edit commits | Change any commit's comment | |
| Edit own commits | Change the comments of your own commits | ✓ |
| Delete commits | Delete any commit | |
| Delete own commits | Delete your own commits | ✓ |
| Download files | Download file contents. Without it, people can't create or update workspaces | ✓ |
| Start merges | Start a merge | ✓ |
| Merge | Commit a merge | ✓ |
| List lock rules | See the **Lock Rules** page | ✓ |
| Edit lock rules | Add, change and delete lock rules | |

**List branches** and **List commits** aren't checked yet. Everyone who can sign in can see every branch and commit, whatever their role.

Renaming and deleting branches isn't in the app yet, so the branch permissions other than **Create branches** don't make a difference so far.

## After an update

When an update adds a new permission, the admin role gets it automatically. Other roles, including developer, don't, so check the **Permissions** list after updating the server.
