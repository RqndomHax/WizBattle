# 🪄 WizBattle
<br>

### You like magic stuff ? This game is for you (UwU)

<br>

# **CROSS PLATFORM GAME** (`Windows`, `Linux`)

<br>

# Install dependencies

> ## Windows

- ### <u><a href="https://win.rustup.rs/">Click to install cargo</a></u>
- ### <u><a href="https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16">Click to install VS2019 build tools</a></u>
- ### <u>`Install cargo-binutils`</u>
    ```bash
    cargo install -f cargo-binutils
    ```
- ### <u>`Install llvm tools`</u>
    ```bash
    rustup component add llvm-tools-preview
    ```

<br>

> ## Linux

- ### <u>`Install cargo`</u>
    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```
- ### <u><a href="https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md">Install _**Bevy**_ dependencies</a></u>
- ### <u>`Install lld`</u>
    > **Ubuntu:** ``sudo apt-get install lld``

    > **Arch:** ``sudo pacman -S lld``

<br>

# How to build ? 

```bash
cargo build --release
```

<br>

# How to run ?

```bash
cargo run --release
```

### Notes

#### This game is using the wonderful *`Bevy`* framework

###### Made with ❤️ by RqndomHax