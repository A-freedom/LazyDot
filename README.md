# Lazydot

Lazydot is a Rust-based tool to help you manage and deploy your dotfiles effortlessly. It automates creating symbolic links for your configuration files, making it easy to keep your environment consistent across multiple machines.

## Features

- Manage your dotfiles in one central repository.
- Automatically create symbolic links from your dotfiles repo to the right places on your system.
- Customize behavior using a simple `config.toml` file.

## Installation
### For install you have a few option
- **you could use the installtion script** `recomended`
    #### just run this script it will download the excutalbe in install it in your ```~/.local/bin/``` and also it will give autocomlation.

    ```bash
    wget some/install/script/yet/to/be/writen
    ```

- **We could just download the excutalbe from the [github release](https://github.com/A-freedom/lazydot/releases) and live you life with it, you cloud install it yourself of use it as portable with you dotfiles repo.**

- **Last you cold just build it form the source. this is if you could not find you platfrom your "cupy_system arch" , do not be afread this is quit easy just flow the flowing instractions.**

    1. Clone the repository:

        ```bash
        git clone https://github.com/A-freedom/lazydot.git
        cd lazydot
        ```

    2. Build the project:
    will you  will need to have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed
        ```bash
        cargo build --release
        ```
    3. you will find the excutalbe named `lazydot` inside `target/release/`. And you can run it using 
        ```bash
        ./target/release/lazydot
        ```
## Usage


### Commands
- `r -- register`  Add one or more dotfile paths to your config. (No files are changed yet.) 
- `u -- unregister` Remove one or more paths from your config. (No files are changed yet.)
- `d -- deploy` - Deploy all symlinks according to the current config. Also cleans up stale links.
- `D -- disable-link` - Unlink specified symlinks and restore real files without touching config.
- `h -- help` — Show a help message

## Use Cases & Examples

- ## **Creating a brand-new dotfiles setup**  

    1- Start by registering the paths you want to manage, no matter where you currently are: 

    
    ```bash
    lazydot register <path1> <path2> ...
    ``` 
    This will add the paths to your  `~/.config/lazydot.toml` (No symlinks are created yet.)
        
    2- Deploy the registered paths:
    ```bash
    lazydot deploy
    ```
    This will:

    - Create symlinks for the listed files.

    - Remove any stale symlinks that are no longer in the config.

- ### **Stopping lazydot from managing a file** 
    If you want to stop managing a file:

    1- Unregister the path from the config:
    ```bash
    lazydot unregister <path1> <path2> ... 
    ```
    2- Deploy again: 
    ```bash
    lazydot deploy
    ```
- ###  **Temporarily disabling a link without touching config**
    Temporarily disabling a link without touching config
    ```bash
    lazydot disable-link <path>
    ```
    This will:

    - Unlink the file.

    - Restore the real file from the dotfolder.

    - Leave the config and dotfolder untouched.
    - also you can you the flag `--all` with `disable-link` to distable all the links manage by your config
    ```bahs
    lazydot disable-link --all  
    ```
- ###  **Editing the Config Manually**  
    You can manually edit the `~/.config/lazydot.toml` file if you prefer.
    ```python
    # lazydot configuration file

    # path to the dotfiles folder (must start with ~/)
    dotfolder_path = "~/mydotfolder"

    # list of dotfile paths to manage (each must start with ~/ or /)
    paths = [
        "~/.config/lazydot.toml"
        "~/.bashrc",
        "~/.zshrc,
        "~/.config/nvim",
    ]

    [defaults]
    # behavior when a duplicate file is found at the destination:
    # - ask: prompt the user to decide
    # - overwritehome: overwrite the file in home with the dotfolder version
    # - overwritedotfile: overwrite the dotfolder copy with the home version
    # - backuphome: backup the home file before overwriting
    # - skip: do nothing and skip the conflict
    on_duplicate = "ask"

    # behavior after a link is disabled (delinked):
    # - remove: remove the file from the dotfolder after restoring it to home (default)
    # - keep: keep the file in the dotfolder even after restoring it to home
    on_delink = "remove"
    ```   
    however lazydot will not immediately apply changes.
    you must run:
    ```bash
    lazydot deploy
    ```
    Afterward to sync your actual files with the new config.  
    **Important**: Manual config edits only change the intent.
    deploy actually applies the changes to your system.
- ###  **About the Current State File**
    LazyDot keeps track of deployed files using a hidden file: `<your dotfolder>/current_state.toml 

    This file:

    - Tracks which paths have been deployed.

    - Allows deploy to clean up stale or removed paths safely.

    If current_state.toml is missing LazyDot will assume a fresh deploy and rebuild it automatically.
    You won't lose your files, but you might have to manually clean old symlinks if things drifted badly.

    #### **Tip**: It’s safe to use version control like `git` to manage your dotfile repo, And add  `current_state.toml` if you want consistent setups across systems,
    but it’s not strictly required.`

- ##  **Using LazyDot with a Cloned Dotfiles Repo**
    Once you've cloned a dotfiles repo, you have a few ways to connect it to LazyDot depending on how your setup is structured.
- ### Option 1: Use lazydot.toml Inside the Repo

    If your repo already contains a lazydot.toml config file, you can run LazyDot directly from inside the repo without needing `~/.config/lazydot.toml`.
    ```bash
    cd ~/dotfiles
    lazydot deploy
    ```
    LazyDot will automatically use `./lazydot.toml` if `~/.config/lazydot`.toml is missing.   
    This allows fully portable dotfiles repos — no need to copy anything into `~/.config/`, Lazydot will create a link for `lazydot.toml`.   
    
- ### Option 2: Generate a LazyDot Template Config

    If your repo doesn’t have a config yet, just run:
    ```bash
    lazydot
    ```
    This will create a default config at `~/.config/lazydot.toml`.
    You can then edit it manually:

    ```python
    # lazydot configuration file

    # path to the dotfiles folder (must start with ~/)
    dotfolder_path = "~/mydotfolder"

    # list of dotfile paths to manage (each must start with ~/ or /)
    paths = [
        "~/.config/lazydot.toml"
        "~/.bashrc",
        "~/.zshrc,
        "~/.config/nvim",
    ]

    [defaults]
    # behavior when a duplicate file is found at the destination:
    # - ask: prompt the user to decide
    # - overwritehome: overwrite the file in home with the dotfolder version
    # - overwritedotfile: overwrite the dotfolder copy with the home version
    # - backuphome: backup the home file before overwriting
    # - skip: do nothing and skip the conflict
    on_duplicate = "ask"

    # behavior after a link is disabled (delinked):
    # - remove: remove the file from the dotfolder after restoring it to home (default)
    # - keep: keep the file in the dotfolder even after restoring it to home
    on_delink = "remove"
    ```   
    however lazydot will not immediately apply changes.
    you must run:
    ```bash
    lazydot deploy
    ```
    Then just deploy:
    ```bash
    lazydot deploy
    ```
- ### Option 3: Register Paths Using Commands

    You can also use the CLI to register paths one by one:
    ``` bash
    lazydot register ~/.bashrc ~/.vimrc
    ```
    - ⚠️ Important: Currently, LazyDot requires the paths to exist inside your $HOME. If the file doesn’t exist yet, register will return an error. This may change in the future.











## Contributing

Contributions are welcome! Fork the repo, make your changes, and open a pull request.

## License

MIT License


