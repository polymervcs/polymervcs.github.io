# Updates and logs

## Find out about updates

The server checks for a new version once a day. When there is one:

- its web page shows the new version in the bar at the top, next to the server's own, and
- if [email is set up](configuration.md#email), every admin is emailed once about it.

The server doesn't update itself.

## Update the server

Back up the server first. See [Data and backups](data.md#back-up-the-server). Then install the new version over the old one, the same way you installed it. The data folder is kept.

- **Debian, Ubuntu and Linux Mint**, **Fedora and openSUSE**: install the new package with `apt` or `dnf` as before. The service restarts with the new version.
- **Arch Linux**: install the new package with `pacman -U`, then restart the service with `sudo systemctl try-restart polymervcs-server`.
- **macOS** and **Windows**: run the new installer. It stops the server, replaces it, and starts it again.
- **Docker**: pull the new image, and recreate the container with the same volume:

  ```bash
  docker pull ghcr.io/polymervcs/polymervcs:latest
  docker rm -f polymervcs
  ```

  Then run it again with the same `docker run` command as before, or run `docker compose pull && docker compose up -d` if you use Compose.

Have your team update their GUI clients and `pvcs` too. While PolymerVCS is in alpha, keep the clients and the server on the same version.

## Logs

The server logs what it's doing, including why it ignored a change to `config.json`, and emails it couldn't send.

| Installed with | Logs |
| --- | --- |
| Linux packages | `journalctl -u polymervcs-server` |
| macOS | `/Library/Logs/PolymerVCS Server/server.log` |
| Windows | The `logs` folder in `C:\Windows\ServiceProfiles\LocalService\.pvcs-server` |
| Docker | `docker logs polymervcs` |

## Uninstall the server

Uninstalling leaves the data folder behind, unless you choose to delete it.

| Installed with | Uninstall |
| --- | --- |
| Debian, Ubuntu and Linux Mint | `sudo apt remove polymervcs-server` |
| Fedora and openSUSE | `sudo dnf remove polymervcs-server`, or `sudo zypper remove polymervcs-server` |
| Arch Linux | `sudo pacman -R polymervcs-server` |
| macOS | `sudo /usr/local/lib/polymervcs-server/uninstall.sh`. Add `--delete-data` to delete the data folder, the log and the `_polymervcs` user too. |
| Windows | **Settings > Apps > Installed apps**. To delete the data folder and logs too, check **Delete the server's repositories, config and logs** in the uninstaller. |
| Docker | `docker rm -f polymervcs`. The data stays in its volume until you delete that, with `docker volume rm polymervcs-data`. |

On Linux, the data folder is in `/var/lib/polymervcs-server` (systemd keeps it in `/var/lib/private/polymervcs-server`). Delete it yourself if you don't need it any more.
