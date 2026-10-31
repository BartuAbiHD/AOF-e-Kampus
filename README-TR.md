<p align="center">
  <img src="assets/icon.png" alt="AOF e-Kampus" width="128" height="128">
</p>

<h1 align="center">AOF e-Kampus</h1>

Türkçe | [English](README.md)

Anadolu Üniversitesi Açıköğretim Fakültesi (AOF) e-Kampus sitesi için
Rust ile yazılmış resmî olmayan masaüstü uygulaması
([tao](https://github.com/tauri-apps/tao) ve [wry](https://github.com/tauri-apps/wry),
Windows'ta WebView2).

## Özellikler

- <https://ekampus.anadolu.edu.tr> adresini yerel bir pencerede açar.
- **Oturumu hatırlar.** Oturum çerezleri kalıcı çereze (30 gün) çevrilir;
  çıkış yapmadığın sürece uygulamayı yeniden açınca oturumun açık kalır.
- Tarayıcı dili varsayılan olarak Türkçe'dir (`tr-TR`). Sağ tık → **Dil / Language** menüsünden 39 dil seçilebilir (uygulama yeniden başlar).
- Küçük boyutlu (1 MB'ın altında) uygulama ve Windows kurulum dosyaları (NSIS ve MSI).

## Gereksinimler

- [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) yüklü Windows 10/11 (Windows 11'de hazır gelir)
- Kaynaktan derlemek için Rust (stable)

## Derleme

```bash
cargo build --release
```

Çıktı: `target/release/aof-ekampus.exe`

### Kurulum dosyaları

`Installers.bat` uygulamayı derler ve `dist/` içine iki kurulum dosyasını üretir.
[WiX Toolset 3.11](https://wixtoolset.org/) (MSI) ve
[NSIS](https://nsis.sourceforge.io/) (kurulum `.exe`) gerekir.

## Veri konumu

Çerezler ve site verileri `%LOCALAPPDATA%\AOF e-Kampus` klasöründe tutulur.
Oturumu tamamen sıfırlamak için bu klasörü sil.

## Sorumluluk reddi

Bu proje Anadolu Üniversitesi ile bağlantılı değildir ve onun tarafından onaylanmamıştır.

## Lisans

[MIT](LICENSE)
