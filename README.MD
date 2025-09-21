# Tauri Setup Guide for Patient Records Information

This guide explains how to set up and run the **Patient Records Information** application built with [Tauri](https://tauri.app/), Vite, and Node.js.

---

## 1. Prerequisites

Before you can run or develop this project, make sure you have the following installed:

### Node.js & npm
- Download and install from [Node.js Official Website](https://nodejs.org/).
- Verify installation:
  ```bash
  node -v
  npm -v
  ```

### Rust (required by Tauri)
- Install Rust using [rustup](https://www.rust-lang.org/tools/install).
- After installation, check:
  ```bash
  rustc --version
  cargo --version
  ```

### Additional Dependencies (depending on your OS)

#### Windows
- Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
- Ensure you have the **Windows 10 SDK** installed.

#### macOS
- Install Xcode from the App Store.
- Run:
  ```bash
  xcode-select --install
  ```

#### Linux (Debian/Ubuntu example)
```bash
sudo apt update
sudo apt install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
```

---

## 2. Creating a New Tauri App (Fresh Setup)

If you want to create this project from scratch, follow these steps:

1. Use the Tauri app generator:
   ```bash
   npm create tauri-app
   ```

2. It will ask you some questions:
   - **Project name** → choose a name (e.g., `patient-records-information`).
   - **Package manager** → choose npm (or yarn/pnpm if you prefer).
   - **Frontend framework** → select React, Vue, Svelte, or Vanilla (for this project: React + Vite).
   - **Typescript** → yes or no depending on your preference.

3. Move into the project folder:
   ```bash
   cd patient-records-information
   ```

4. Install dependencies:
   ```bash
   npm install
   ```

5. Run in development mode:
   ```bash
   npm run tauri dev
   ```

6. To build the application (creates an installer/binary):
   ```bash
   npm run tauri build
   ```

---

## 3. Cloning This Repository (Recommended)

If you already have the project from GitHub, you don’t need to create a new Tauri app. Instead:

1. Clone the repository:
   ```bash
   git clone https://github.com/Alaric-Jeff/Records_Information.git
   ```

2. Navigate into the project:
   ```bash
   cd patient-records-information
   ```

3. Install dependencies:
   ```bash
   npm install
   ```

4. Run in development mode:
   ```bash
   npm run tauri dev
   ```

5. Build production app:
   ```bash
   npm run tauri build
   ```

---

## 4. Notes

- The `src-tauri` folder contains the Rust backend for Tauri.
- The `src` folder contains the frontend code (React/Vite).
- You must have both **Node.js** and **Rust** installed, otherwise the build will fail.
- For troubleshooting, see [Tauri Docs](https://tauri.app/).

---

## 5. Quick Commands

```bash
# Development
npm run tauri dev

# Production build
npm run tauri build
```

