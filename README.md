# Harboor Sweep

<p align="center">
  <img src="public/logo.png" alt="Harboor Sweep Logo" width="200"/>
</p>

**Harboor Sweep** is a simple desktop application built with [Tauri v2](https://v2.tauri.app/)
and [Vue.js](https://vuejs.org/). It is designed to identify and manage active ports and their processes.

> **Note:** This project is still under active development. Contributions and feedback are welcome!

## Features

- 🚀 Lightweight and fast desktop app using Tauri and Rust
- ⚡ Modern frontend powered by Vue 3
- 🔒 Offline, Secure and privacy-focused
- 🛠️ Easy to build and customize

## Getting Started

### Prerequisites

Before you can build or run **Harboor Sweep** locally, you need to have:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Node.js](https://nodejs.org/) (version 18 or newer recommended)
- [pnpm](https://pnpm.io/) (or npm / yarn)
- Tauri dependencies based on your platform:
    - **Linux**: `libgtk-3-dev`, `webkit2gtk-4.0`, etc.
    - **macOS**: Xcode Command Line Tools
    - **Windows**: Visual Studio with Desktop Development workload

> ### You can find the full list of Tauri v2 prerequisites [here](https://v2.tauri.app/start/prerequisites/). Make sure your environment is ready before proceeding.

### Installation

Clone the repository:

```bash
git clone https://github.com/LazyDoomSlayer/harboor-sweep.git
cd harboor-sweep
```

Install the frontend dependencies:

```bash
pnpm install --frozen-lockfile
```

### Development

To start the app in development mode (hot-reload + Tauri):

```bash
pnpm tauri dev
```

### Building for Production

To build the app for production:

```bash
pnpm tauri build
```

The final binaries will be generated inside the `src-tauri/target` folder.

## Project Structure

```plaintext
harboor-sweep/
├── src/            # Vue.js frontend source code
├── src-tauri/      # Tauri backend configuration and Rust source
├── public/         # Static assets
├── package.json    # Project metadata and scripts
├── tauri.conf.json # Tauri app configuration
└── README.md       # Project documentation
```

## Contributing

If you find a bug or have a feature request, feel free to open an issue or submit a pull request!  
Follow conventional commit guidelines if possible.

## License

This project is licensed under the [MIT License](LICENSE).
