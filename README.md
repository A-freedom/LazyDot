# Lazydot

Lazydot is a Rust-based tool designed to manage and deploy your dotfiles efficiently. It automates the process of
setting up symbolic links for your configuration files, making it easier to maintain consistent environments across
multiple systems.

## Features

- **Dotfile Management**: Organize your configuration files in a centralized repository.
- **Automated Symlinking**: Automatically create symbolic links from your dotfiles repository to their proper locations
  in your system.
- **Configurable Setup**: Customize the behavior and structure through a ```config.toml``` file.

## Installation

To install lazydot, perform the following steps:

1. Clone the Repository:

    ```bash
        git clone https://github.com/A-freedom/lazydot.git
        cd lazydot
    ```

2. Build the Project:

    ```bash
        cargo build --release
    ```

3. Set Up Configuration:

    - Edit the ```~/.config/lazydot.toml``` file to specify your dotfiles and their target locations.
    - Example configuration:
      ```toml
      dotfolder_path = "~/mydotfolder"
      paths = [
         "~/bashrc",
         "~/.config/nvim",
         ".config/alacritty/alacritty.toml"
      ]
      ```
    - Note : paths can be ether an existing file or folder, And it has to be inside the Home directory

## Usage

Lazydot provides several subcommands to manage your dotfiles:

```text
   Usage: lazydot <COMMAND>

   Commands:
     add-path      Add a path
     remove-path   Remove a path
     apply-config  Apply config
     un-link-all   Unlinking all the paths
     un-link       unlink a paths or a list of paths
     help          Print this message or the help of the given subcommand(s)

   Options:
     -h, --help     Print help
     -V, --version  Print 
```

And you can run it but running the executable

```bash 
    ./target/release/lazydot
```

## Contributing
Contributions are welcome. Fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License.
