# AOF e-Kampus

[Türkçe](README-TR.md) | English

An unofficial desktop wrapper for the Anadolu University Open Education
Faculty (AOF) e-Kampus website, written in Rust with
[tao](https://github.com/tauri-apps/tao) and [wry](https://github.com/tauri-apps/wry)
(WebView2 on Windows).

## Features

- Opens <https://ekampus.anadolu.edu.tr> in a native window.
- **Remembers your login.** Session cookies are re-saved as persistent cookies
  (30 days), so the app stays signed in after restarting until you log out.
- Browser language defaults to Turkish (`tr-TR`). Right-click → **Dil / Language** lets you pick from 39 languages (the app restarts to apply it).
- Tiny binary (under 1 MB) and Windows installers (NSIS and MSI).

## Requirements

- Windows 10/11 with the [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) (preinstalled on Windows 11)
- Rust (stable) to build from source

## Build

```bash
cargo build --release
```

The executable is `target/release/aof-ekampus.exe`.

### Installers

`Installers.bat` builds the app and produces both installers in `dist/`.
It needs [WiX Toolset 3.11](https://wixtoolset.org/) (MSI) and
[NSIS](https://nsis.sourceforge.io/) (setup `.exe`).

## Data location

Cookies and site data are stored in `%LOCALAPPDATA%\AOF e-Kampus`.
Delete this folder to sign out completely.

## Disclaimer

This project is not affiliated with or endorsed by Anadolu University.

## License

[MIT](LICENSE)
