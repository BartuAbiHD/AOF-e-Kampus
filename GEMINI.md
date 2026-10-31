# GEMINI.md

Guidance for AI coding agents working in this repository.

## Project

AOF e-Kampus: a Rust (tao + wry/WebView2) desktop wrapper around
<https://ekampus.anadolu.edu.tr>. Windows only.

## Layout

- `src/main.rs` – the whole app: window, webview, cookie persistence, language.
- `build.rs` – embeds `assets/icon.ico` into the exe (Windows).
- `installer/aof-ekampus.wxs` – WiX 3 MSI definition.
- `installer/aof-ekampus.nsi` – NSIS installer script.
- `Installers.bat` – builds the exe, MSI and NSIS setup into `dist/`.
- `assets/` – icons (`icon.ico` is multi-size 16–256).

## Commands

- Build: `cargo build --release`
- Run: `cargo run --release`
- Installers: `Installers.bat` (needs WiX 3.11 and NSIS)

## Conventions

- Keep the app small: avoid adding dependencies without a clear need.
- Code comments and user-facing text are in Turkish; docs exist in English and Turkish
  (`README.md`, `README-TR.md`) and must be kept in sync.
- Session persistence works by re-saving session cookies with a 30-day expiry
  (`persist_cookies`). Do not remove the periodic/close-time save.
- The webview data directory is `%LOCALAPPDATA%\AOF e-Kampus`.
- Bump the version in `Cargo.toml`, the `.wxs` and the `.nsi` together.
- Do not commit `target/` or `dist/`.
