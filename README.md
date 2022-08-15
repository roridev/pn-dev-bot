# pn-dev-bot
Powernukkit's developer bot.  
Used mainly by the core team for getting information using the GitHub API.  

## Features

- Rich Issue/PR embeds.  
- Listing bugs based on their verification status (Confirmed/Unconfirmed).  
- `git` command generation for Pull Requests.  

## Planned features

- Two-way integration (Discord <-> Github) where issues can be closed/created via commands.  
- Displaying (even more) information for Issues and Pull Requests.  

## Setting up

Requirements
---
- Install [`rustup`](https://rustup.rs)  

Run the following commands: 
```
rustup install nightly
cd pn-dev-bot
cargo build
```

IDE Support
---
IntelliJ IDEA: [Official Extension by `Jetbrains`](https://www.jetbrains.com/rust/).  
Visual Studio Code: [Marketplace Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust).  
emacs: Follow [this guide](https://robert.kra.hn/posts/rust-emacs-setup/).
