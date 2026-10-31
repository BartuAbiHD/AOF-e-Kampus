#![windows_subsystem = "windows"]

use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder},
};
#[cfg(windows)]
use wry::WebViewBuilderExtWindows;
use wry::{cookie::time::OffsetDateTime, WebContext, WebViewBuilder};

const START_URL: &str = "https://ekampus.anadolu.edu.tr";
const ALLOWED_HOST: &str = "anadolu.edu.tr";
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

fn host_of(url: &str) -> Option<String> {
    let rest = url.split_once("://")?.1;
    let host = rest.split(['/', '?', '#']).next()?;
    Some(host.split(':').next()?.to_string())
}

/// Oturum çerezlerinin (süresi olmayan) tarayıcı kapanınca silinmemesi için
/// bunları kalıcı çerez olarak yeniden yazar.
fn persist_cookies(webview: &wry::WebView, hosts: &Mutex<HashSet<String>>) {
    let expires = OffsetDateTime::now_utc() + wry::cookie::time::Duration::days(SESSION_DAYS);
    let hosts: Vec<String> = hosts.lock().unwrap().iter().cloned().collect();
    for host in hosts {
        let Ok(cookies) = webview.cookies_for_url(&format!("https://{host}/")) else {
            continue;
        };
        for mut c in cookies {
            if c.expires().is_none() {
                c.set_expires(expires);
                let _ = webview.set_cookie(&c);
            }
        }
    }
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("AOF e-Kampus")
        .with_inner_size(LogicalSize::new(1200.0, 800.0))
        .with_window_icon(load_icon())
        .build(&event_loop)
        .expect("pencere oluşturulamadı");

    // Kalıcı veri klasörü: çerezler ve oturum burada saklanır.
    let mut context = WebContext::new(Some(data_dir()));

    let hosts: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    hosts.lock().unwrap().insert(host_of(START_URL).unwrap());
    let nav_hosts = hosts.clone();

    let builder = WebViewBuilder::new_with_web_context(&mut context).with_url(START_URL);

    // Tarayıcı dili Türkçe: Accept-Language ve site dili tr-TR olur.
    #[cfg(windows)]
    let builder = builder.with_additional_browser_args(
        "--lang=tr-TR --accept-lang=tr-TR,tr --disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection",
    );

    let webview = builder
        .with_navigation_handler(move |url| {
            if let Some(h) = host_of(&url) {
                if h.ends_with(ALLOWED_HOST) {
                    nav_hosts.lock().unwrap().insert(h);
                }
            }
            true
        })
        .build(&window)?;

    let mut next_save = Instant::now() + SAVE_INTERVAL;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(next_save);

        match event {
            Event::NewEvents(_) if Instant::now() >= next_save => {
                persist_cookies(&webview, &hosts);
                next_save = Instant::now() + SAVE_INTERVAL;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                persist_cookies(&webview, &hosts);
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
