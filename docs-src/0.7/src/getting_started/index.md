# Getting Started

## Pick an Editor

We recommend using [VSCode](https://code.visualstudio.com) since Dioxus ships with its [own VSCode extension](https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus). Our build tool `dx` is standalone and is meant to be used through a terminal.

Most editors support the [Rust-Analyzer LSP plugin](https://rust-analyzer.github.io) which provides appropriate syntax highlighting, code navigation, folding, and more. You can follow the [installation instructions](https://rust-analyzer.github.io/manual.html#installation) for your editor of choice: [VSCode](https://rust-analyzer.github.io/manual.html#vs-code), [Zed](https://rust-analyzer.github.io/manual.html#zed), [Emacs](https://rust-analyzer.github.io/manual.html#emacs), or [Vim](https://rust-analyzer.github.io/manual.html#vimneovim).

## Install Rust

Head over to [https://rust-lang.org](http://rust-lang.org) and install the Rust compiler (preferably using `rustup`). Once installed, make sure you add the `stable` toolchain and any relevant toolchains (ie wasm32-unknown-unknown for web apps):

```sh
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
```

We strongly recommend going through the [official Rust book](https://doc.rust-lang.org/book/ch01-00-getting-started.html) _completely_. However, we hope that a Dioxus app can serve as a great first Rust project.

We've put a lot of care into making Dioxus syntax familiar and easy to understand, so you won't need deep knowledge of async, lifetimes, or smart pointers until you start building complex Dioxus apps.


## Install the Dioxus CLI

Dioxus ships with its own build tool that leverages `cargo` to provide integrated hot-reloading, bundling, and development servers for web and mobile. You can download the prebuilt binary with the following command:

```sh
curl -fsSL https://dioxuslabs.com/install | bash
```

You can also download with `cargo-binstall`:

```sh
cargo binstall dioxus-cli --force
```

If you want to build the CLI from source, you can install it with the following command:

```sh
cargo install dioxus-cli
```

> 📣 Installing from source can take up to 10 minutes and requires several dependencies. We *strongly* recommend downloading the prebuilt binaries.

If you get an OpenSSL error on installation, ensure the dependencies listed [here](https://docs.rs/openssl/latest/openssl/#automatic) are installed.

## Platform-specific dependencies

Most platforms don't require any additional dependencies, but if you are targeting desktop, you might need to install additional dependencies.

You can use the `dx doctor` command to see if `dx` can properly understand your install. This command helps provide insight into missing toolchains and tools required for cross-platform development.

### macOS

There are no extra dependencies for macOS! However, if you want to build iOS apps, read the [iOS section](#ios) below.

### Windows

Windows apps depend on WebView2 – a library that should be installed in all modern Windows distributions.

If you have Edge installed, then Dioxus will work fine. If you _don't_ have WebView2, then you can [install it through Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/). Microsoft provides 3 options:

1. A tiny "evergreen" _bootstrapper_ that fetches an installer from Microsoft's CDN.
2. A tiny _installer_ that fetches WebView2 from Microsoft's CDN.
3. A statically linked version of WebView2 in your final binary for offline users.

We recommend using Option 1.

### Linux

WebView Linux apps require WebkitGtk and xdotool. When distributing, these should be part of your dependency tree in your `.rpm` or `.deb`.

If you run into issues, make sure you have all the basics installed.

For Ubuntu, make sure everything is installed:

```sh
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

For arch:
```sh
sudo pacman -Syu
sudo pacman -S --needed \
  webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  libappindicator-gtk3 \
  librsvg \
  xdotool
```

For all other Linux targets, [check the Tauri docs which cover the same dependencies](https://tauri.app/start/prerequisites/#linux).

In addition to the Tauri docs, for Fedora:
```sh
sudo dnf install libxdo-devel
```

### WSL

While doable, it can be tricky to setup development in WSL for Dioxus desktop. Not everything has been figured out and some stuff may not work.

Here are the steps we used to get Dioxus running through WSL:

1. Update your kernel to the latest version and update WSL to version 2.
2. Add `export DISPLAY=:0` to `~/.zshrc`
3. Install Tauri's Linux dependencies found [here](https://beta.tauri.app/start/prerequisites/).
4. For file dialogs to work, you need to install a fallback like `zenity`

When running Dioxus desktop on WSL, you may get warnings from `libEGL`. There is currently no way to silence these, but the app should still render.

### iOS

Building iOS apps requires a device running macOS with XCode installed.

Download and install XCode from one of the following places:
- [Mac App Store](https://apps.apple.com/gb/app/xcode/id497799835?mt=12)
- [Apple Developer website](https://developer.apple.com/xcode/resources/)

You will need to download the iOS SDK and install some simulators.

For more details, we recommend reading the [dedicated guide for iOS development](../guides/platforms/mobile.md).

### Android

Android apps require the Android SDK and NDK to be installed. This can be a substantial amount of setup, so we recommend reading the [dedicated guide for Android development](../guides/platforms/mobile.md).
