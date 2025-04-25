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
- `r -- register` - Register one or more dotfile paths in your config 
- `u -- unregister` - Remove one or more paths from your config
- `d -- deploy` - Create or update all symlinks according to current config
- `c -- clean` - Remove specified symlink(s); if none given, it removes all when --all is used
- `h -- help` — Show a help message

### Use Cases & Examples

- **Creating brand new dotfiles folder**  
this is the easyest one, all what you have to do is start add the your paths to be registered using the command `register` flowed by the paths. you could to this regardless on your current path.  
 
    ```bash
    lazydot register <path1> <path2> .etc
    ``` 
    this will add the paths to the `~/.config/lazydot.toml`  
    now you cold just deploy this will create the link and sync
    ```bash
    lazydot deploy
    ```
    if you wnat to remove a path from the monterd path you can just use `unrigister`




















## Contributing

Contributions are welcome! Fork the repo, make your changes, and open a pull request.

## License

MIT License



3. Configure your dotfiles by editing `~/.config/lazydot.toml`:

    ```toml
    dotfolder_path = "~/mydotfolder"
    paths = [
       "~/.bashrc",
       "~/.config/nvim",
       "~/.config/alacritty/alacritty.toml"
    ]
    ```

> **Note:** Paths must be inside your home directory and can point to files or folders.