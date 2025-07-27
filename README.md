<div align="center">

<img src="assets/icon.png" width="100" height="100" alt="Logo">

# branchy text editor


</div>

**Branchy Text Editor** is a modern and performant text editor built with the **Tauri framework**, featuring a responsive frontend powered by **SolidJS**. Get accurate and fast syntax highlighting thanks to **Tree-sitter**, making your coding experience smoother and more efficient.

---

![](app.png)

## 🛠️Getting started
### 📦Prerequisites
- [Node.js](https://nodejs.org/en/download/) (v18.0.0 or later)
- [Rust](https://www.rust-lang.org/tools/install) (v1.70
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/) (v2.0.0 or later)
- [Pnpm](https://pnpm.io/installation) (v8.0.0 or later)

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/TodayNightt/branchy-text-editor
   ```
   
2. Navigate to the project directory:
   ```bash
    cd branchy-text-editor
    ```
3. Build the executable:
    ```bash
   cargo tauri build
    ```
4. Install the application:
    ```bash
    ./src-tauri/target/release/branchy.exe
    ```

## Currently supported platform

- Windows x86_64 (only)

## Currently supported text (With syntax hightlighting)

- Rust
- JavaScript
- Java
- Html
- Json
