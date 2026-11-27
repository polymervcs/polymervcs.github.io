# Lock rules

Files like textures, models, audio and scenes can't be merged. When two people change the same one, one of them has to throw their work away. Lock rules say which files PolymerVCS should lock, so only one person can change each of them at a time.

> [!NOTE]
> File locking isn't available yet. You can set up lock rules now, but no files are locked until a later version.

Manage them on the **Lock Rules** page of the server's web UI. Everyone with **List lock rules** can see them, and **Edit lock rules** is needed to change them. See [Roles and permissions](roles.md).

## Add lock rules

Under **Add lock rules**:

1. Add the **File patterns** of the files to lock, one at a time, or click a preset to add the usual ones for a kind of file or a game engine. Each pattern becomes its own rule.
2. Under **Destination branches**, check at least one branch.
3. Optionally, add **Excluded branches**.
4. Click **Add lock rule**.

The rules are listed in a table, where you can edit or delete each one.

## File patterns

A pattern matches files or folders by path, like `*.png` or `**/Library/*.prefs`:

- `*` matches anything except `/`, and `?` matches one character.
- `**` matches any number of folders. It has to be a whole part of the path on its own, as in `**/Library`, not `a**`.
- `[abc]` matches one of the characters in the brackets.
- `{png,jpg}` matches any of the comma-separated words. Groups can't be nested.
- Folders are separated by `/`, never `\`, and `.` and `..` can't be used.

A pattern starting with `!` never locks what it matches. Later rules win over earlier ones, so put `!` patterns after the ones they make exceptions to. **Sort** keeps them last.

## Presets

The presets add patterns for:

- **3D**: interchange formats such as `.fbx`, `.glb` and `.usd`, and Blender, 3ds Max, Maya, Cinema 4D, Houdini and ZBrush files.
- **Images**: image formats, and Photoshop, GIMP, Krita, Affinity, Aseprite and Substance files.
- **Audio**: audio formats, digital audio workstation projects, and FMOD and Wwise files.
- **Video**: video formats, including Bink, and video editing projects.
- **Fonts**: font formats and font editor projects.
- **Unity**: Unity's own assets, such as `.unity`, `.prefab`, `.asset` and `.mat`.
- **Unreal**: `.uasset` and `.umap`.
- **Godot**: Godot's binary scenes and resources, but not `.tscn` and `.tres`, which can be merged.

## Branches

Each rule has at least one **destination branch**, chosen from the repository's branches. Deleting a branch takes it out of the rules it's a destination of.

**Excluded branches** are optional. Give each one as a full branch name, such as `main/experiment`, or with wildcards, such as `main/experiments/*` or `*/art-concepts/*`. Don't start or end them with `/`.
