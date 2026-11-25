# Ignoring files

Build output, caches and editor settings usually don't belong in the repository. To keep files like these out of your pending changes, list them in a file called `.pvcsignore`.

`.pvcsignore` files work like Git's `.gitignore` files:

- A `.pvcsignore` can be in any folder of the workspace. Its patterns apply to what's in that folder and the folders under it.
- Each line is a pattern. Blank lines, and lines starting with `#`, are skipped.
- `*` matches anything except `/`, `?` matches one character, and `**` matches any number of folders.
- A pattern ending in `/` only matches folders.
- A pattern with a `/` at the start or in the middle is relative to the `.pvcsignore`'s folder. Otherwise it matches at any depth.
- A pattern starting with `!` includes again what an earlier pattern ignored. Nothing inside an ignored folder can be included again, though.
- Later lines win over earlier ones, and a `.pvcsignore` in a deeper folder wins over one further up.
- Patterns ignore case on Windows and macOS, and are case-sensitive on Linux.

The `.pvcs` folder is always ignored. Changes to a `.pvcsignore` take effect straight away: newly ignored files drop out of **Pending Changes**, and files that are no longer ignored appear.

## Ignoring only applies to new files

Files that are already in the repository stay in it, even if they match a pattern, and changes to them are still pending changes. To take a file out of the repository, delete it, commit the deletion, and add it to `.pvcsignore` so it stays out when it comes back.

## Examples

For a Unity project, in the project's folder:

```text
/[Ll]ibrary/
/[Tt]emp/
/[Oo]bj/
/[Bb]uild/
/[Bb]uilds/
/[Ll]ogs/
/[Uu]ser[Ss]ettings/
*.csproj
*.sln
```

For an Unreal Engine project:

```text
Binaries/
DerivedDataCache/
Intermediate/
Saved/
.vs/
*.sln
```

For a Godot project:

```text
.godot/
```
