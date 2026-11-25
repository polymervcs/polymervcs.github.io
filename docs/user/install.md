# Install the app

The PolymerVCS app and the `pvcs` command-line client are on the [downloads page](/downloads). On Windows and macOS, one installer adds both. On Linux, each has its own package, so install the ones you want.

In the file names below, `<version>` stands for the version you downloaded, such as `0.1.0`.

## Windows

Run `polymervcs-<version>-x64-setup.exe`. It needs 64-bit Windows 10 or later.

- The app is installed in `C:\Program Files\PolymerVCS`, unless you choose another folder, and added to the Start menu as **PolymerVCS**.
- `pvcs` is installed in the `bin` folder there, which setup adds to your `PATH`. Open a new terminal to use it.
- The app needs the Microsoft Edge WebView2 Runtime. Most Windows 10 and 11 PCs already have it. If yours doesn't, setup downloads and installs it, which needs an internet connection.

To uninstall, use **Settings > Apps > Installed apps**.

## macOS

Open `polymervcs-<version>-universal.pkg`. It runs on Apple silicon and Intel Macs with macOS 11 or later.

- The app is installed in Applications, as **PolymerVCS**.
- `pvcs` is installed in `/usr/local/bin`.

To uninstall, move PolymerVCS to the Trash, then remove `pvcs`:

```bash
sudo rm /usr/local/bin/pvcs
```

## Debian, Ubuntu and Linux Mint

```bash
sudo apt install ./polymervcs-gui_<version>_amd64.deb ./polymervcs-cmd_<version>_amd64.deb
```

Keep the `./`. Without it, apt looks for a package with that name in its repositories, instead of using the file.

## Fedora and openSUSE

```bash
sudo dnf install ./polymervcs-gui-<version>.x86_64.rpm ./polymervcs-cmd-<version>.x86_64.rpm
```

On openSUSE, run `sudo zypper install` with the same files.

## Arch Linux

```bash
sudo pacman -U polymervcs-gui-<version>-x86_64.pkg.tar.zst polymervcs-cmd-<version>-x86_64.pkg.tar.zst
```

On Linux, the app is in your applications menu as **PolymerVCS**, and `pvcs` is in `/usr/bin`.

## Check which version you have

```bash
pvcs --version
```

In the app, the version is at the top of **Settings**, next to **Check for update**.

## Updates

PolymerVCS doesn't update itself. To see whether there's a newer version, click **Check for update** in the app's [Settings](tools.md), or run `pvcs update`. To update, download the new installer or package, and install it the same way as before. Your workspaces, settings and sign-ins are kept.
