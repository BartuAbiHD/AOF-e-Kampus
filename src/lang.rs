//! Tarayıcı dili ayarı ve sağ tık menüsündeki "Dil" alt menüsü.

use std::{fs, path::Path};

use webview2_com::{
    ContextMenuRequestedEventHandler, CustomItemSelectedEventHandler,
    Microsoft::Web::WebView2::Win32::*,
};
use windows::core::{Interface, HSTRING, PCWSTR};
use wry::WebViewExtWindows;

pub const DEFAULT_LANG: &str = "tr-TR";

/// (dil kodu, kendi dilindeki adı)
pub const LANGUAGES: &[(&str, &str)] = &[
    ("tr-TR", "Türkçe"),
    ("en-US", "English (US)"),
    ("en-GB", "English (UK)"),
    ("de-DE", "Deutsch"),
    ("fr-FR", "Français"),
    ("es-ES", "Español"),
    ("it-IT", "Italiano"),
    ("pt-BR", "Português (Brasil)"),
    ("pt-PT", "Português (Portugal)"),
    ("nl-NL", "Nederlands"),
    ("sv-SE", "Svenska"),
    ("da-DK", "Dansk"),
    ("nb-NO", "Norsk"),
    ("fi-FI", "Suomi"),
    ("pl-PL", "Polski"),
    ("cs-CZ", "Čeština"),
    ("sk-SK", "Slovenčina"),
    ("hu-HU", "Magyar"),
    ("ro-RO", "Română"),
    ("bg-BG", "Български"),
    ("el-GR", "Ελληνικά"),
    ("ru-RU", "Русский"),
    ("uk-UA", "Українська"),
    ("az-AZ", "Azərbaycanca"),
    ("kk-KZ", "Қазақша"),
    ("uz-UZ", "Oʻzbekcha"),
    ("ar-SA", "العربية"),
    ("fa-IR", "فارسی"),
    ("he-IL", "עברית"),
    ("hi-IN", "हिन्दी"),
    ("bn-BD", "বাংলা"),
    ("ur-PK", "اردو"),
    ("th-TH", "ไทย"),
    ("vi-VN", "Tiếng Việt"),
    ("id-ID", "Bahasa Indonesia"),
    ("ms-MY", "Bahasa Melayu"),
    ("zh-CN", "简体中文"),
    ("zh-TW", "繁體中文"),
    ("ja-JP", "日本語"),
    ("ko-KR", "한국어"),
];

fn file(dir: &Path) -> std::path::PathBuf {
    dir.join("language.txt")
}

/// Kayıtlı dili okur; yoksa ya da geçersizse Türkçe döner.
pub fn load(dir: &Path) -> String {
    fs::read_to_string(file(dir))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| LANGUAGES.iter().any(|(c, _)| c == s))
        .unwrap_or_else(|| DEFAULT_LANG.to_string())
}

pub fn save(dir: &Path, code: &str) {
    let _ = fs::create_dir_all(dir);
    let _ = fs::write(file(dir), code);
}

/// WebView2 başlatma argümanları (tarayıcı dili ve Accept-Language).
pub fn browser_args(code: &str) -> String {
    let base = code.split('-').next().unwrap_or(code);
    format!(
        "--lang={code} --accept-lang={code},{base} --disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection"
    )
}

/// Sağ tık menüsünün sonuna "Dil / Language" alt menüsünü ekler.
/// Bir dil seçilince `on_select` dil koduyla çağrılır.
pub fn install_menu(
    webview: &wry::WebView,
    current: String,
    on_select: impl Fn(&'static str) + Clone + 'static,
) -> windows::core::Result<()> {
    let core = webview.webview().cast::<ICoreWebView2_11>()?;

    let handler = ContextMenuRequestedEventHandler::create(Box::new(move |sender, args| {
        let (Some(sender), Some(args)) = (sender, args) else {
            return Ok(());
        };
        unsafe {
            let env = sender
                .cast::<ICoreWebView2_2>()?
                .Environment()?
                .cast::<ICoreWebView2Environment9>()?;
            let items = args.MenuItems()?;

            let title = HSTRING::from("Dil / Language");
            let parent = env.CreateContextMenuItem(
                PCWSTR(title.as_ptr()),
                None,
                COREWEBVIEW2_CONTEXT_MENU_ITEM_KIND_SUBMENU,
            )?;
            let children = parent.Children()?;

            for (i, (code, name)) in LANGUAGES.iter().enumerate() {
                let label = HSTRING::from(*name);
                let item = env.CreateContextMenuItem(
                    PCWSTR(label.as_ptr()),
                    None,
                    COREWEBVIEW2_CONTEXT_MENU_ITEM_KIND_CHECK_BOX,
                )?;
                item.SetIsChecked(*code == current)?;

                let cb = on_select.clone();
                let code: &'static str = code;
                let selected = CustomItemSelectedEventHandler::create(Box::new(move |_, _| {
                    cb(code);
                    Ok(())
                }));
                let mut token = Default::default();
                item.add_CustomItemSelected(&selected, &mut token)?;
                children.InsertValueAtIndex(i as u32, &item)?;
            }

            let mut count = 0u32;
            items.Count(&mut count)?;
            items.InsertValueAtIndex(count, &parent)?;
        }
        Ok(())
    }));

    let mut token = Default::default();
    unsafe { core.add_ContextMenuRequested(&handler, &mut token)? };
    Ok(())
}
