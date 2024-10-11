# Rainbow

## Installation 
You will firstly need to install the Rust language. See https://www.rust-lang.org/tools/install.

Now that you have Rustup installed switch to the nightly version
of the tools using the following terminal command:

`rustup default nightly`

You can install Rainbow by running the following:

`cargo install --path $HOME/devops/rainbow`

Or if you prefer to just run it straight from source:

`cargo run`

By default, `cargo install` will place the binary into `$HOME/.cargo/bin`.
Add this to your path by adding the following line to your .bshrc or .zshrc file (substituting $HOME appropriately):

`export PATH="$HOME/.cargo/bin$PATH"`

Use the help menu to find out what you can do from there:

`rainbow --help`

## Note
Rainbow uses truecolor, your terminal will need to support this.  eg. [iTerm 2](https://iterm2.com) or [Alacritty](https://github.com/alacritty/alacritty)
