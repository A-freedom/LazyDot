# LazyDot

> **Tame your dotfiles. Stop silent breakage. Deploy with confidence.**

---

## What’s the Problem?

**Stow is lazy.** It doesn’t track what you actually stowed —  
New files? Not linked. Deleted files? Still hanging around.  
Your dotfiles silently **rot** over time.

---

## What’s the Solution?

**LazyDot** **explicitly tracks** your dotfiles, keeping your environment consistent and portable across machines.  
It’s simple: you add files → LazyDot manages them → No surprises.

---

## Philosophy

LazyDot follows the Unix philosophy: **Do one thing, and do it well.**
- Lazydot manages dotfiles by explicitly tracking and syncing them using a configuration file.
- Features outside this narrow scope (e.g., automatic discovery, file encryption, template generation) will **never** be implemented.
- Lazydot prioritizes clarity, simplicity, and reliability over feature bloat.

---

## Features

- ✅ **Explicit tracking** — only manage what you add.
- ✅ **Sync safely** — add, remove, fix broken links anytime.
- ✅ **Cross-platform binaries** — install in seconds.
- ✅ **Simple CLI** — no bloated configs needed.
- ✅ **Portable repos** — use local lazydot.toml without home config.

---

## Installation

### Option 1: One-line Script (Recommended)

```bash
curl -s https://raw.githubusercontent.com/A-freedom/lazydot/main/install.sh | bash
```
This installs LazyDot to `~/.local/bin/` and sets up shell autocompletion.

---

### Option 2: Download Prebuilt Binary

Grab the latest release from [GitHub Releases](https://github.com/A-freedom/lazydot/releases) and move it anywhere in your `$PATH`.

---

### Option 3: Build from Source

If your platform isn’t supported:
```bash
git clone https://github.com/A-freedom/lazydot.git
cd lazydot
cargo build --release
```
The binary will be in `target/release/lazydot`.

---

## Quick Usage

**Add dotfiles:**

```bash
lazydot add ~/.bashrc ~/.config/nvim
lazydot sync
```

You can also add all files inside a directory at once using a wildcard (`*`):

```bash
lazydot add ~/.config/*
```

**Remove tracked dotfiles:**

```bash
lazydot remove ~/.bashrc
lazydot sync
```

The wildcard can also be used with `remove`:

```bash
lazydot remove ~/.config/*
```

---

## Commands Overview

| Command        | Shortcut | Description                                                         |
|:---------------|:---------|:--------------------------------------------------------------------|
| `add`          | `-a`     | Add one or more paths to config (no symlinks yet, must sync after). |
| `remove`       | `-r`     | Remove paths from config (must sync after to apply changes).        |
| `sync`         | `-s`     | Create/update symlinks and clean up stale links.                    |
| `disable-link` | `-d`     | Unlink one or all paths without changing config.                    |
| `help`         | `-h`     | Show help message.                                                  |

---

## Behavior Clarifications

1. **Explicit Dotfile Management**
   - Lazydot does **NOT** automatically discover files. Users must manually specify the files they want to manage.
   - This is a **feature, not a bug**. Lazydot ensures the user is in full control.

2. **Conflict Resolution**
   - If a file already exists, Lazydot uses the `on_duplicate` option.
     - Default behavior: Ask the user how to proceed.
     - Other behaviors: Overwrite or skip, based on user settings.

3. **Unlinking Files**
   - Removing a path from the config does **not** instantly delete symlinks.
   - After running `lazydot sync`, outdated links will be automatically fixed or cleaned.
   - No manual intervention is needed.

4. **Version Control**
   - Users are **strongly encouraged** to manage their `.config/lazydot.toml` using Git.
   - Planned Feature: **Optional auto-commit** after each successful sync if changes are detected in the config.

5. **Security (Encryption)**
   - Lazydot does **not** implement encryption.
   - Managing secrets should be handled separately, preferably through Git solutions or external tools.

6. **Templating**
   - Lazydot does **not** support dynamic templating of config files.
   - Files are symlinked exactly as they are.

---

## Use Cases & Examples

### 🚀 **Setting Up Dotfiles from Scratch**

```bash
lazydot add ~/.bashrc ~/.config/nvim
lazydot sync
```

---

### ✂️ **Stop Managing a File**

```bash
lazydot remove ~/.zshrc
lazydot sync
```

---

### 🔌 **Temporarily Disable a Link**

Disable a specific link:
```bash
lazydot disable-link ~/.bashrc
```

Disable **all** managed links:
```bash
lazydot disable-link --all
```

---

### 🛠️ **Edit Config Manually**

Edit `~/.config/lazydot.toml`:

```toml
dotfolder_path = "~/mydotfolder"

paths = [
  "~/.bashrc",
  "~/.config/nvim",
]

[defaults]
on_duplicate = "ask"
on_delink = "remove"
```

Then:

```bash
lazydot sync
```

---

### 📦 **Using LazyDot with a Cloned Repo**

If you've cloned a repo that contains a `lazydot.toml` file:

- LazyDot checks for a global config at `~/.config/lazydot.toml` first.
- If not found, it will look for `./lazydot.toml` locally.
- It can auto-link your local config to the global one if needed.

Run LazyDot from within your repo:

```bash
cd ~/dotfiles
lazydot sync
```

---

## About Adding Non-Existent Files

You can add paths even if they don't exist in your home directory, as long as they exist inside your dotfolder.  
If not, LazyDot will error out.

---

## About the Current State File

LazyDot keeps a `current_state.toml` inside your dotfolder to:
- Track deployed files.
- Allow safe cleaning and resyncing.

**If missing:** It rebuilds automatically during sync.

---

## Future Work Plan

- Implement `lazydot status` to show pending changes without applying them.
- Implement `lazydot restore-config` to restore missing or broken configs.
- Auto-commit option for Git after syncs.
- More documentation improvements.

---

## Why LazyDot Over Stow?

|                                      | `stow` | `lazydot` |
|:-------------------------------------|:-------|:----------|
| Tracks files explicitly              | ❌      | ✅         |
| Detects missing/broken links         | ❌      | ✅         |
| Portable repo support                | ❌      | ✅         |
| Automatic recovery from manual edits | ❌      | ✅         |

---

## Contributing

Contributions are welcome!

- Found a bug? **Open an issue.**
- Have an idea? **Request a feature.**
- Want to help shape the future? **Participate in discussions.**

Fork it. Improve it. Make dotfiles suck less.

---

## License

[Apache License 2.0](./LICENSE-2.0.txt)

---

# TL;DR

> If you want a tool that \"does everything,\" use something else.
> 
> Some suggestions:
> - [chezmoi](https://www.chezmoi.io/)
> - [yadm](https://yadm.io/)
> - [dotdrop](https://github.com/deadc0de6/dotdrop)
> - [rcm](https://github.com/thoughtbot/rcm)

> If you want **full manual control** over your dotfiles with **zero magic, zero surprises**, welcome to LazyDot.

---


