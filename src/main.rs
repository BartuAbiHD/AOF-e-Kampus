#![windows_subsystem = "windows"]

mod lang;

use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Icon, WindowBuilder},
};
#[cfg(windows)]
use wry::WebViewBuilderExtWindows;
use wry::{cookie::time::OffsetDateTime, WebContext, WebViewBuilder};

enum UserEvent {
    Restart,
}

const START_URL: &str = "https://ekampus.anadolu.edu.tr";
const SESSION_DAYS: i64 = 30;
const SAVE_INTERVAL: Duration = Duration::from_secs(60);

fn data_dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join("AOF e-Kampus")
}

fn load_icon() -> Option<Icon> {
    let img = image::load_from_memory(include_bytes!("../assets/icon.png"))
        .ok()?
        .into_rgba8();
    let (w, h) = img.dimensions();
    Icon::from_rgba(img.into_raw(), w, h).ok()
}

/// Oturum çerezlerinin (süresi olmayan) tarayıcı kapanınca silinmemesi için
/// bunları kalıcı çerez olarak yeniden yazar.
fn persist_cookies(webview: &wry::WebView) {
    let expires = OffsetDateTime::now_utc() + wry::cookie::time::Duration::days(SESSION_DAYS);
    let Ok(cookies) = webview.cookies() else {
        return;
    };
    for mut c in cookies {
        // wry oturum çerezlerini `Expiration::Session` olarak bildirir.
        if c.expires_datetime().is_none() {
            c.set_expires(expires);
            let _ = webview.set_cookie(&c);
        }
    }
}

fn main() -> wry::Result<()> {
    // Yeniden başlatma sonrası eski sürecin kapanması için kısa bekleme.
    if std::env::args().any(|a| a == "--delay") {
        std::thread::sleep(Duration::from_millis(1500));
    }

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("AOF e-Kampus")
        .with_inner_size(LogicalSize::new(1200.0, 800.0))
        .with_window_icon(load_icon())
        .build(&event_loop)
        .expect("pencere oluşturulamadı");

    // Kalıcı veri klasörü: çerezler ve oturum burada saklanır.
    let mut context = WebContext::new(Some(data_dir()));

    let builder = WebViewBuilder::new_with_web_context(&mut context).with_url(START_URL);

    // Tarayıcı dili (varsayılan Türkçe); sağ tık menüsünden değiştirilebilir.
    let current_lang = lang::load(&data_dir());
    #[cfg(windows)]
    let builder = builder.with_additional_browser_args(lang::browser_args(&current_lang));

    let webview = builder
        .build(&window)?;

    let menu_proxy = proxy.clone();
    let _ = lang::install_menu(&webview, current_lang, move |code| {
        lang::save(&data_dir(), code);
        let _ = menu_proxy.send_event(UserEvent::Restart);
    });

    let mut next_save = Instant::now() + SAVE_INTERVAL;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(next_save);

        match event {
            Event::NewEvents(_) if Instant::now() >= next_save => {
                persist_cookies(&webview);
                next_save = Instant::now() + SAVE_INTERVAL;
            }
            Event::UserEvent(UserEvent::Restart) => {
                // Dil, tarayıcı başlarken uygulandığı için uygulama yeniden açılır.
                persist_cookies(&webview);
                if let Ok(exe) = std::env::current_exe() {
                    let _ = std::process::Command::new(exe).arg("--delay").spawn();
                }
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                persist_cookies(&webview);
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
