# Install the server

PolymerVCS Server runs on Linux, macOS and Windows, or in Docker. The installers set it up as a service that starts with the computer. Downloads are on the [downloads page](/downloads), and the file names below are for the latest version, {{version}}.

Once it's installed, the server listens on port 3000, on every network interface. Then [set it up](setup.md) straight away, as whoever opens its web UI first becomes its admin.

## Debian, Ubuntu and Linux Mint

Download and install:

```bash
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-server_{{version}}_amd64.deb
sudo apt install ./polymervcs-server_{{version}}_amd64.deb
```

The package installs the `polymervcs-server` systemd service, and starts it. Its data is in `/var/lib/polymervcs-server/.pvcs-server`.

If the server's firewall is on, open port 3000. With ufw:

```bash
sudo ufw allow 3000/tcp
```

## Fedora and openSUSE

Download and install:

```bash
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-server-{{version}}.x86_64.rpm
sudo dnf install ./polymervcs-server-{{version}}.x86_64.rpm
```

On openSUSE, run `sudo zypper install` with the same file. The package installs the `polymervcs-server` systemd service, and starts it. Its data is in `/var/lib/polymervcs-server/.pvcs-server`.

To open port 3000 in firewalld:

```bash
sudo firewall-cmd --permanent --add-port=3000/tcp
sudo firewall-cmd --reload
```

## Arch Linux

Download and install:

```bash
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-server-{{version}}-x86_64.pkg.tar.zst
sudo pacman -U polymervcs-server-{{version}}-x86_64.pkg.tar.zst
```

Arch packages leave starting services to you, so start it, and have it start with the computer, with:

```bash
sudo systemctl enable --now polymervcs-server
```

Its data is in `/var/lib/polymervcs-server/.pvcs-server`.

## macOS

Download and open the installer:

```bash
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-server-{{version}}-universal.pkg
open polymervcs-server-{{version}}-universal.pkg
```

PolymerVCS Server runs on Apple silicon and Intel Macs with macOS 11 or later.

The server runs as the `com.polymervcs.server` launchd daemon, as a hidden user called `_polymervcs` that the installer creates. If the macOS firewall is on, the installer lets the server accept incoming connections.

- Its data is in `/Library/Application Support/PolymerVCS Server/.pvcs-server`.
- Its log is `/Library/Logs/PolymerVCS Server/server.log`.

## Windows

Download and run the installer, in PowerShell:

```powershell
curl.exe -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-server-{{version}}-x64-setup.exe
.\polymervcs-server-{{version}}-x64-setup.exe
```

Type `curl.exe`, not `curl`, which is a different command in Windows PowerShell.

PolymerVCS Server needs 64-bit Windows 10 or later.

The server runs as the **polymervcs-server** Windows service, as the Local Service account, and starts automatically a little after Windows does. Setup adds a Windows Firewall rule called **PolymerVCS Server**, which lets it accept connections on domain and private networks. Connections on networks Windows treats as public are still blocked.

- It's installed in `C:\Program Files\PolymerVCS Server`, unless you choose another folder.
- Its data is in `C:\Windows\ServiceProfiles\LocalService\.pvcs-server`.
- Its logs are in the `logs` folder there.

## Docker

The image is `ghcr.io/polymervcs/polymervcs`. The server keeps its data in `/root/.pvcs-server` in the container, so keep that in a volume, or it's lost along with the container:

```bash
docker run -d --name polymervcs --restart unless-stopped \
  -p 3000:3000 \
  -v polymervcs-data:/root/.pvcs-server \
  -e BASE_URL=http://vcs.example.com \
  ghcr.io/polymervcs/polymervcs:latest
```

Or with Docker Compose:

```yaml
services:
  polymervcs:
    image: ghcr.io/polymervcs/polymervcs:latest
    restart: unless-stopped
    ports:
      - "3000:3000"
    environment:
      BASE_URL: http://vcs.example.com
    volumes:
      - polymervcs-data:/root/.pvcs-server

volumes:
  polymervcs-data:
```

Set `BASE_URL` to the address people reach the server at, without the port. Other settings can be given as environment variables too. See [Configuration](configuration.md).

Keep the container's port and the published port the same, such as `3000:3000`. Invite links use the server's own port, so with `8080:3000`, for example, they'd point at the wrong one. To use another port, set `PORT` as well, such as `-e PORT=8080 -p 8080:8080`.

## Run it by hand

The server is one program, `pvcsserver` (`pvcsserver.exe` on Windows), with a `public` folder next to it. Run as yourself, it keeps its data in `.pvcs-server` in your home folder. `pvcsserver --help` lists its options, which are described in [Configuration](configuration.md).

| Installed with | Program |
| --- | --- |
| Linux packages | `/usr/lib/polymervcs-server/pvcsserver` |
| macOS | `/usr/local/lib/polymervcs-server/pvcsserver` |
| Windows | `C:\Program Files\PolymerVCS Server\pvcsserver.exe` |

Stop the service before running the server by hand. Both would use the same port, and they'd each have their own data folder, as they run as different users.
