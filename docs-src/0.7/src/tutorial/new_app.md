
## Create a new project

You can create a new Dioxus project by running the following command and following the prompts:

```sh
dx new hot_dog
```

![dxnew](/assets/06_docs/dx_new_06.mp4)

You'll need to select a template to use to get started.

- Bare-bones: a very simple setup with just a `main.rs` an and `assets` folder.
- Jumpstart: a scaffolded app with components, views, and suggested structure.
- Workspace: a full cargo workspace setup with different crates per platform.

We're going to use the bare-bones template for *HotDog* since our app will be quite simple.

- Select "false" when asked if you want to create a fullstack website.
- Select "false" for the router, though we *will* eventually add the router to the app.
- Select "true" for TailwindCSS.
- Select "Web" as the default platform.

> 📣 You don't need `dx new` to create new Dioxus apps! Dioxus apps are Rust projects and can also be built with tools like cargo.


## Running the project

Once the project is generated, you can start it with the following command:

```sh
cd hot_dog
dx serve
```

![Serve](/assets/06_docs/dx_serve_06.mp4)

This will start the cargo build and launch a web server to serve your app. If you visit the "serve" address (in this case, `http://127.0.0.1:8080`), then you'll receive a loading screen in your browser:

![loading](/assets/06_docs/hotdog_loading.png)

Once the app is loaded, you should be greeted with the default Dioxus template app:

![app](/assets/06_docs/default_dioxus_app.png)

Congrats! You have your very first Dioxus app.

## Structure of the app

Open the app in your editor and take a look at its structure:

```sh
├── Cargo.lock
├── Cargo.toml
├── Dioxus.toml
├── README.md
├── assets
│   ├── favicon.ico
│   ├── header.svg
│   └── main.css
└── src
    └── main.rs
```

All Rust apps are comprised of a root `Cargo.toml` with a `main.rs` file located in the `src` folder. Our CLI `dx` pre-filled these files with the `dioxus` dependency and some starter code for us to get building quickly.

Assets in Dioxus can be placed anywhere in the project, but we suggest leaving them in the `assets` folder.

### The Cargo.toml

The `Cargo.toml` outlines the dependencies to our app and specifies compiler settings. All Rust apps are *compiled*: we execute the Rust tool `cargo` which aggregates our `.rs` files together and generates a final binary executable (like a `.exe`) that runs our app.

All Dioxus apps will include `dioxus` as a dependency:

```toml
[dependencies]
dioxus = { version = "0.7.0" }
```

The prebuilt Dioxus templates initialize different cargo features for your app. `dx` will use these to decide which cargo features to enable when you specify the `--platform` feature. For example, if you use `dx serve --platform desktop` to build your app for desktop, `dx` will call `cargo build --no-default-features --features desktop`.

```toml
[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
```

### Dioxus.toml

The `Dioxus.toml` file contains Dioxus-specific configuration for used by bundling and deploying. We won't need to configure the `Dioxus.toml` for our app just yet.

### Assets Folder

To include assets in your Dioxus app, you'll want to use the `asset!()` macro that we'll cover later in the [Styling and Assets](assets.md) chapter. You can include assets from anywhere within your app's file tree, but we recommend using the pregenerated `assets` folder.

### tailwind.css

Dioxus has built-in support for TailwindCSS! When serving and building your app, `dx` automatically runs the TailwindCSS CLI if it detects a `tailwind.css` at the root of your app. The default output for the tailwind compiler is in the assets folder.

### main.rs

Finally, the `main.rs`. The `main.rs` file is the entrypoint of our app, containing the `fn main` function. All Rust executables start their life at `main`.

The `main` of our HotDog app looks like this:

```rust
{{#include ../docs-router/src/doc_examples/guide_new_app.rs:new_app}}
```

The `launch` function calls the platform-specific `launch` function depending on which feature (web/desktop/mobile) is enabled on `dioxus`. `launch` accepts a root component, typically called `App`.

We'll cover components more in-depth in the [next chapter](component.md).

## Resetting to Basics

The bare-bones template provides basic starter code for our app. However, we want to start *truly* from scratch, so we'll wipe away the `Hero` component and empty the `App` component to its basics:

```rust
{{#include ../docs-router/src/doc_examples/guide_new_app.rs:new_app_full}}
```
