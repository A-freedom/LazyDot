
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

## Use Cases & Examples

### 🚀 **Setting Up Dotfiles from Scratch**

```bash
lazydot add ~/.bashrc ~/.config/nvim
lazydot sync
```

- This registers your files into the config and creates symlinks.
- **By default**, your actual dotfiles will be expected to exist inside the dotfolder path.
- The default dotfolder is: `~/mydotfolder`
- You can change this path inside your `~/.config/lazydot.toml` if needed.

---

### ✂️ **Stop Managing a File**

```bash
lazydot remove ~/.zshrc
lazydot sync
```
The link is safely removed.

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
on_duplicate = "ask"  # options: ask, overwritehome, overwritedotfile, backuphome, skip
on_delink = "remove"  # options: remove, keep
```

Then:

```bash
lazydot sync
```

*Changes are applied only after syncing.*

---

### 📦 **Using LazyDot with a Cloned Repo**

If you've cloned a repo that contains a `lazydot.toml` file:

- **LazyDot checks for a global config at `~/.config/lazydot.toml` first.**
  - If global config exists, LazyDot uses it directly.
  - If global config is missing, LazyDot looks for a local config (`./lazydot.toml`).
    - **If local config found:** LazyDot uses it and creates a symlink from the global path to this local config.
    - **If local config not found:** LazyDot generates a fresh default config at the global path.

To use it, simply run LazyDot from within your repo directory:

```bash
cd ~/dotfiles
lazydot sync
```

---

## About Adding Non-Existent Files

You can add paths even if they don't exist in your home directory, as long as the corresponding files already exist inside your configured dotfolder.  
Simply run:

```bash
lazydot add path_inside_your_dotfolder
```

**Important:**
- A working global config must exist with the correct `dotfolder_path` pointing to your repository.
- If no config exists, LazyDot will create a new one with a default `dotfolder_path`, which might not match your cloned repo unless you update it manually.

---

## About the Current State File

LazyDot keeps a hidden `current_state.toml` inside your dotfolder to:
- Track deployed files.
- Allow safe cleaning and resyncing.

**If missing:** LazyDot will rebuild it automatically on the next sync.

**Tip:** It’s strongly recommended to use version control like `git` for your entire dotfiles repository,  
including the `current_state.toml` if you want absolutely consistent setups across machines.

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

> If you’re tired of `stow` silently breaking your dotfiles behind your back,  
> **LazyDot is the fix you've been waiting for.**

---

