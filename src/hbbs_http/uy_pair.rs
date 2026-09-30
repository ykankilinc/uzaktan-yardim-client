//! Uzaktan Yardım: Teknisyen uygulamasını panele eşleştirme.
//!
//! Panel "Bu bilgisayarı eşleştir" dediğinde `<uygulama>://eslestir/<kod>` linkini açar.
//! Uygulama kodu panelin `/api/teknisyen/eslestir` ucuna gönderir; dönen kişiye özel
//! anahtar `access_token` olarak saklanır ve her bağlantı isteğinde `token` alanıyla
//! hbbs'e gider (hbbs bu anahtarın hangi cihazlara izinli olduğunu denetler).

use hbb_common::{
    bail,
    config::{Config, LocalConfig},
    log, ResultType,
};
use serde_json::{json, Value};

const PAIR_PATH: &str = "eslestir/";

/// `<uygulama>://eslestir/<kod>` ise kodu döndürür.
pub fn code_from_link(arg: &str) -> Option<String> {
    let rest = arg.strip_prefix(&crate::get_uri_prefix())?;
    let code = rest.strip_prefix(PAIR_PATH)?.trim_end_matches('/');
    let valid = (6..=32).contains(&code.len()) && code.chars().all(|c| c.is_ascii_alphanumeric());
    valid.then(|| code.to_owned())
}

fn pair(code: &str) -> ResultType<String> {
    let api = crate::common::get_api_server(
        Config::get_option("api-server"),
        Config::get_option("custom-rendezvous-server"),
    );
    if api.is_empty() {
        bail!("Panel adresi tanımlı değil");
    }
    let body = json!({
        "code": code,
        "id": Config::get_id(),
        "pc_name": crate::common::hostname(),
    });
    let text = crate::post_request_sync(format!("{api}/api/teknisyen/eslestir"), body.to_string(), "")?;
    let v: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
    match v["token"].as_str() {
        Some(token) if !token.is_empty() => {
            LocalConfig::set_option("access_token".to_owned(), token.to_owned());
            Ok(v["username"].as_str().unwrap_or("").to_owned())
        }
        _ => bail!(
            "{}",
            v["error"].as_str().unwrap_or("Panel eşleştirmeyi kabul etmedi")
        ),
    }
}

/// Eşleştirir ve sonucu kullanıcıya gösterir (arayüz açılmaz).
pub fn pair_and_notify(code: &str) {
    let (ok, text) = match pair(code) {
        Ok(user) => {
            log::info!("Teknisyen eşleştirildi: {user}");
            (true, format!(
                "Bu bilgisayar eşleştirildi{}.\n\nArtık panelden bağlanabilirsiniz.",
                if user.is_empty() { String::new() } else { format!(" ({user})") }
            ))
        }
        Err(e) => {
            log::warn!("Teknisyen eşleştirilemedi: {e}");
            (false, format!("Eşleştirme başarısız: {e}\n\nPanelden yeni bir eşleştirme linki alın."))
        }
    };
    #[cfg(windows)]
    message_box(&text, ok);
    #[cfg(not(windows))]
    println!("{text} ({ok})");
}

#[cfg(windows)]
fn message_box(text: &str, ok: bool) {
    use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_OK};
    let wide = |s: &str| s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    let (text, caption) = (wide(text), wide("Uzaktan Yardım Teknisyen"));
    let icon = if ok { MB_ICONINFORMATION } else { MB_ICONERROR };
    unsafe { MessageBoxW(std::ptr::null_mut(), text.as_ptr(), caption.as_ptr(), MB_OK | icon) };
}
