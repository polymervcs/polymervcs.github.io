# Installing clients

The PolymerVCS GUI and CMD clients can be found on the [downloads page](/downloads).

On Linux, each has its own package, so install the ones you want. On Windows and macOS, the installer adds both.

## Debian, Ubuntu and Linux Mint

```bash
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-gui_{{version}}_amd64.deb
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-cmd_{{version}}_amd64.deb
sudo apt install ./polymervcs-gui_{{version}}_amd64.deb ./polymervcs-cmd_{{version}}_amd64.deb
```

## Fedora and openSUSE

```bash
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-gui-{{version}}.x86_64.rpm
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-cmd-{{version}}.x86_64.rpm
sudo dnf install ./polymervcs-gui-{{version}}.x86_64.rpm ./polymervcs-cmd-{{version}}.x86_64.rpm
```

On openSUSE, run `sudo zypper install` with the same files.

## Arch Linux

```bash
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-gui-{{version}}-x86_64.pkg.tar.zst
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-cmd-{{version}}-x86_64.pkg.tar.zst
sudo pacman -U polymervcs-gui-{{version}}-x86_64.pkg.tar.zst polymervcs-cmd-{{version}}-x86_64.pkg.tar.zst
```

On Linux, the GUI client is in your applications menu as **PolymerVCS**, and `pvcs` is in `/usr/bin`.

## macOS

In a Terminal windows, download and open the installer:

```bash
curl -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-{{version}}-universal.pkg
open polymervcs-{{version}}-universal.pkg
```

PolymerVCS runs on Apple silicon and Intel Macs with macOS 11 or later.

- The GUI client is installed in Applications, as **PolymerVCS**.
- `pvcs` is installed in `/usr/local/bin`.

To uninstall, move PolymerVCS to the Trash, then remove `pvcs`:

```bash
sudo rm /usr/local/bin/pvcs
```

## Windows

In a PowerShell window, download and run the installer:

```powershell
curl.exe -fLO https://github.com/polymervcs/polymervcs/releases/download/v{{version}}/polymervcs-{{version}}-x64-setup.exe
.\polymervcs-{{version}}-x64-setup.exe
```

PolymerVCS needs 64-bit Windows 10 or later.

- By default, The GUI client is installed in `C:\Program Files\PolymerVCS` and added to the Start menu as **PolymerVCS**.
- `pvcs` is installed in the `bin` folder there, which setup adds to your `PATH`. Open a new terminal to use it.
- The GUI client needs the Microsoft Edge WebView2 Runtime. Most Windows 10 and 11 PCs already have it. If yours doesn't, setup downloads and installs it, which needs an internet connection.

To uninstall, use **Settings > Apps > Installed apps**.

## Updates

To see whether there's a newer version, click **Check for update** in the GUI client's [Settings](tools.md), or run <code class="copy">pvcs update</code>. To update, download the new installer or package, and install it the same way as before. Your workspaces, settings and sign-ins are kept.
