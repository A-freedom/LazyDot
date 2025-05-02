# LazyDot

> **Tame your dotfiles. Stop silent breakage. Deploy with confidence.**
---

## TL;DR

Tired of `stow` silently breaking your dotfiles?
**LazyDot** gives you full control by explicitly managing what you track — no surprises.

```bash
lazydot add ~/.bashrc ~/.config/nvim
lazydot sync
```

Done. Repeatable, portable, no hidden magic.

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#quick-usage">Quick Usage</a> •
  <a href="#pro-tips">Pro Tips</a>
</p>  


## What’s the Problem?

`stow` links the content of folders, not the folders themselves. It mirrors the structure, but doesn’t track specific
files.

That means:

- If you delete a file from your home, `stow` won’t know.
- If you add a new file to your dot repo, `stow` won’t sync it unless it was already there during `stow`'s operation.

LazyDot is different:

- It tracks explicit paths — file or folder.
- If you add a folder, **the entire folder is symlinked**.
- The OS ensures that any file changes inside that folder are reflected automatically.

No tracking magic — just smarter design.

---

## What’s the Solution?

**LazyDot** tracks files explicitly. You tell it what to manage — and it makes sure those exact links are created and
maintained. Nothing more. Nothing less.

- You’re in control
- You see what’s broken
- You fix it with a sync

No silent failure. No forgotten files.

---

## Features

- ✅ **Explicit tracking** — you must add files manually
- ✅ **Portable** — works with local or global config
- ✅ **Safe syncing** — never blindly overwrites without your consent
- ✅ **Status and diagnostics** — detect broken or missing links
- ✅ **Cross-platform binaries** — works out of the box

---

## Philosophy

LazyDot is built on the Unix philosophy:

> **Do one thing and do it well.**

That’s why features like:

- 🔒 encryption
- 📄 templating

…are **out of scope**. Use the right tools for those jobs.

LazyDot is here to do one thing: manage dotfiles **correctly**.

---

## Installation

### Option 1: One-line Script

```bash
curl -s https://raw.githubusercontent.com/A-freedom/lazydot/main/install.sh | bash
```

Installs to `~/.local/bin/` and sets up autocompletion.

### Option 2: GitHub Releases

- Download binary from [Releases](https://github.com/A-freedom/lazydot/releases)
- Add to your `$PATH`, or drop it in your dotfiles repo for portability

### Option 3: Build from Source

```bash
git clone https://github.com/A-freedom/lazydot.git
cd lazydot
cargo build --release
```

---

## Creating a New Dotfile Repo

1. Create your dotfolder:

  ```bash
  mkdir ~/dotfiles
  ```  

2. Set it in config (`~/.config/lazydot.toml`):

  ```toml
  dotfolder_path = "~/dotfiles"
  ```  

3. Move your files into it, then register them:

  ```bash
  lazydot add ~/.bashrc ~/.zshrc
  lazydot sync
  ```   

---

## Starting from an Existing Repo

1. Clone your repo:

```bash
git clone git@github.com:you/dotfiles.git ~/dotfiles
```

2. Run LazyDot from that directory:

```bash
cd ~/dotfiles
lazydot sync
```

It will:

- Use `~/.config/lazydot.toml` if available
- Otherwise link to local `./lazydot.toml`

---

## Adding Paths Without Existing Home Files

You can add paths that don’t exist in `$HOME` **as long as** the corresponding file exists in the dotfolder.
Useful when bootstrapping a new system or pre-building your config:

```bash
lazydot add ~/.config/wayland/hyprland.conf
```

---

## Commands Overview

| Command        | Shortcut | Description                                                      |
|----------------|----------|------------------------------------------------------------------|
| `add`          | `-a`     | Add one or more paths to config (run `sync` to apply)            |
| `remove`       | `-r`     | Remove paths from config (run `sync` to apply)                   |
| `sync`         | `-s`     | Apply changes, create or update symlinks, and clean broken links |
| `disable-link` | `-d`     | Unlink dotfiles temporarily without changing config              |
| `status`       | `-t`     | View link status of all tracked files                            |
| `check`        | `-c`     | Validate link health and print a report                          |
| `help`         | `-h`     | Show help message                                                |

---

## Quick Usage

### Add and Sync

Register files to manage and link them:

```bash
lazydot add ~/.bashrc ~/.config/nvim
lazydot sync
```

### Remove and Sync

Untrack a file and clean up the symlink:

```bash
lazydot remove ~/.bashrc
lazydot sync
```

### Inspect

Check status and validate links:

```bash
lazydot status
lazydot check
```

---

## Behavior Clarifications

### 🔍 Explicit File Management

- LazyDot does NOT auto-discover files
- You must explicitly add what you want tracked

### ⚠️ Conflict Resolution

- If a target file exists, LazyDot uses `on_duplicate` behavior
- Default: ask
- Options: overwrite, backup, skip, etc

### 🔄 Sync Required

- All changes require a `lazydot sync` to apply
- Removing files from config does NOT auto-remove links

### 🔒 Security

- LazyDot does NOT handle secrets or encryption
- Use `git-crypt`, `sops`, or similar tools

### 🧩 Templating

- LazyDot does not support file rendering or templating
- Symlinks the exact file as-is

---

## Configuration File

`~/.config/lazydot.toml` or `./lazydot.toml`

```toml
# Required: where dotfiles are stored

dotfolder_path = "~/dotfiles"

# Dotfiles to track
paths = [
    "~/.bashrc",
    "~/.config/nvim/init.vim",
    "~/.config/lazydot.toml"
]

# Optional behavior settings
[defaults]
on_duplicate = "ask"     # ask, overwritehome, overwritedotfile, skip, backuphome
on_delink = "remove"      # remove, keep
```

---

## About the Current State File

LazyDot uses `.current_state.toml` to remember which files are currently linked. It enables:

- Safer syncs
- Smarter cleanup

If missing or deleted, LazyDot will regenerate it on next sync.

---

## Pro Tips

Use Git to version your dotfiles.

This gives you an incredibly powerful recovery mechanism. Because LazyDot tracks state and your repo tracks content, you
can:

- revert accidental changes
- restore missing links
- undo massive mistakes

No matter how badly you mess up your home directory or dotfolder — you can bring it all back.

```bash
git init
git add .
git commit -m "first commit"
echo 'lazydot_state.toml' >> .gitignore
```

If you mess something up:

```bash
git reset --hard
lazydot sync
```

Your system is restored.

---

## License

[Apache License 2.0](./LICENSE-2.0.txt)

---
