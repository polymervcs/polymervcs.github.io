# Configuration

The server's settings are in a file called `config.json`, in its data folder:

| Installed with | Config file |
| --- | --- |
| Linux packages | `/var/lib/polymervcs-server/.pvcs-server/config.json` |
| macOS | `/Library/Application Support/PolymerVCS Server/.pvcs-server/config.json` |
| Windows | `C:\Windows\ServiceProfiles\LocalService\.pvcs-server\config.json` |
| Docker | `/root/.pvcs-server/config.json`, in the container |
| Run by hand | `.pvcs-server/config.json`, in your home folder |

The server creates it the first time it starts:

```json
{
  "port": 3000,
  "login_type": "SelfManaged",
  "base_url": "http://localhost/",
  "smtp": null,
  "s3": null
}
```

The file can hold email and storage passwords, so on Linux and macOS, only the server's own user can read it. Edit it as an administrator, such as with `sudo nano`.

## Changes apply straight away

The server watches `config.json`. When the file changes, the server checks the new settings, then restarts itself with them, without its service restarting. Anything it was in the middle of, such as an upload, is cut off.

If the new settings don't work, such as a file that isn't valid JSON, or a port that's already in use, the server logs why and keeps running with the old settings. Check the [logs](maintenance.md#logs) after a change.

## Environment variables and command-line options

Each setting can also be given as an environment variable, or as an option when you run `pvcsserver`. When the server starts, it takes the settings in `config.json`, replaces them with any environment variables, then with any options, and saves the result back to `config.json`.

That means environment variables and options are remembered: removing one later doesn't change the setting back. Edit `config.json` to do that. It also means they apply again at every start, so if one is set, change it there rather than in `config.json`, or the file's change only lasts until the next restart.

Environment variables are how you configure the [Docker image](install.md#docker). For a systemd service, add them with:

```bash
sudo systemctl edit polymervcs-server
```

```ini
[Service]
Environment=BASE_URL=http://vcs.example.com
```

## Settings

| Setting | Environment variable | Option | Default |
| --- | --- | --- | --- |
| `port` | `PORT` | `--port` | `3000` |
| `base_url` | `BASE_URL` | `--base-url` | `http://localhost/` |
| `login_type` | `LOGIN_TYPE` | `--login-type` | `SelfManaged` |
| `smtp.server_url` | `SMTP_SERVER_URL` | `--smtp-server-url` | |
| `smtp.username` | `SMTP_USERNAME` | `--smtp-username` | |
| `smtp.password` | `SMTP_PASSWORD` | `--smtp-password` | |
| `s3.endpoint` | `S3_ENDPOINT` | `--s3-endpoint` | |
| `s3.bucket_name` | `S3_BUCKET_NAME` | `--s3-bucket-name` | |
| `s3.access_key_id` | `S3_ACCESS_KEY_ID` | `--s3-access-key-id` | |
| `s3.secret_access_key` | `S3_SECRET_ACCESS_KEY` | `--s3-secret-access-key` | |
| `s3.region` | `S3_REGION` | `--s3-region` | |
| `s3.prefix` | `S3_PREFIX` | `--s3-prefix` | |

### `port`

The TCP port the server listens on, on every network interface. People include it in the server's URL, such as `http://vcs.example.com:3000`.

### `base_url`

The address people reach the server at, such as `http://vcs.example.com`. Invite links, and the update emails sent to admins, are made from it. The server always adds its own `port` to it, so leave the port out. One given here is ignored.

### `login_type`

How people sign in. The only choice so far is `SelfManaged` (`self-managed` as an option), where the server keeps its own accounts and passwords.

## Email

With email set up, the server emails invites, and tells admins when there's a new version. Without it, nothing is emailed, and you send each invite link yourself.

```json
"smtp": {
  "server_url": "smtp.example.com",
  "username": "vcs@example.com",
  "password": "the account's password"
}
```

- `server_url` is the host name of your mail provider's SMTP server. The server connects to it on port 465, with TLS, and checks its certificate. Mail servers that only offer port 587 with STARTTLS aren't supported yet.
- `username` is the account to sign in to the mail server as. It has to be an email address, as it's also the address the emails come from, with "PolymerVCS" as the sender's name.
- `password` is that account's password. Providers that use two-factor sign-in usually need an app password here.

All three are needed. Giving only some of them stops the server from starting.

## File storage

The server keeps file contents in its data folder unless you give it an S3-compatible bucket, such as Amazon S3, Cloudflare R2, Backblaze B2 or MinIO. Its databases stay in the data folder either way.

```json
"s3": {
  "endpoint": "https://s3.us-east-1.amazonaws.com",
  "bucket_name": "example-polymervcs",
  "access_key_id": "AKIA...",
  "secret_access_key": "...",
  "region": "us-east-1",
  "prefix": "polymervcs"
}
```

- `endpoint` is the storage provider's URL. It has to start with `https://`. Buckets are addressed by path, as `<endpoint>/<bucket_name>`.
- `bucket_name`, `access_key_id` and `secret_access_key` are the bucket and the keys to reach it with. The keys need to be able to read, write and delete objects in the bucket.
- `region` is optional. It's `us-east-1` when it's left out.
- `prefix` is optional. It's a folder in the bucket to keep everything in, so one bucket can hold several servers' files.

The endpoint, bucket and both keys are all needed. Giving only some of them stops the server from starting.

Changing where files are stored doesn't move the ones already stored. Set up S3 before anyone commits, or see [Data and backups](data.md#move-files-to-s3) to move them.
