# Data and backups

## What's in the data folder

Everything the server keeps is in its data folder, `.pvcs-server`. Where that is depends on how it was installed. See the table in [Configuration](configuration.md).

| Path | What it holds |
| --- | --- |
| `config.json` | The server's settings. See [Configuration](configuration.md). |
| `db/` | The repository's database: its branches, commits and file history. |
| `blobs/` | The contents of every file version, split into chunks, unless the server [stores them in S3](configuration.md#file-storage). |
| `auth/auth.db` | People's accounts, roles, permissions, invites and sign-ins. |
| `auth/auth.secret` | The key the server signs sign-ins with. Anyone who has it could sign in as anyone, so keep it private. |

## Back up the server

Stop the server, copy its whole data folder somewhere safe, then start it again. Stopping it first means the databases are copied in a consistent state. If the server stores files in S3, back up the bucket too, as it isn't in the data folder.

| Installed with | Stop | Start |
| --- | --- | --- |
| Linux packages | `sudo systemctl stop polymervcs-server` | `sudo systemctl start polymervcs-server` |
| macOS | `sudo launchctl bootout system/com.polymervcs.server` | `sudo launchctl bootstrap system /Library/LaunchDaemons/com.polymervcs.server.plist` |
| Windows | `Stop-Service polymervcs-server` | `Start-Service polymervcs-server` |
| Docker | `docker stop polymervcs` | `docker start polymervcs` |

On Windows, run the commands in PowerShell as an administrator, or use the **Services** app.

On Linux and macOS, copy with `sudo cp -a`, or another tool that keeps files' owners, so a restored copy still belongs to the server's user. With Docker, copy the folder out of the stopped container:

```bash
docker cp polymervcs:/root/.pvcs-server ./polymervcs-backup
```

To restore a backup, stop the server, put the copy in place of its data folder, and start it again. With Docker, that's `docker cp ./polymervcs-backup/. polymervcs:/root/.pvcs-server`.

## Deleted branches and commits

When a branch or commit is deleted, the server remembers that for 30 days, so that workspaces that haven't been opened for a while still find out about it. Then it forgets it.

## Move files to S3

The server doesn't move files when you [change where they're stored](configuration.md#file-storage), so do it yourself:

1. Stop the server.
2. Copy everything in `blobs/` to the bucket, under the prefix if you're using one, keeping the same folder structure. With the AWS CLI, for example:

   ```bash
   aws s3 sync blobs/ s3://example-polymervcs/polymervcs/
   ```

   For a provider other than Amazon, add `--endpoint-url` with the same endpoint as in the server's settings.

3. Add the S3 settings to `config.json`, then start the server.

Once you've checked everything works, you can delete `blobs/`.
