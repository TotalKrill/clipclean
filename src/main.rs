extern crate clipboard;
extern crate url;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use clipclean::*;
use url::*;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let cleaner = UrlCleaner::default();
    let mut previous = String::new();
    loop {
        let string = ctx.get_contents();

        if let Ok(s) = string {
            if previous != s {
                previous = s.clone();
                let parsed = Url::parse(&s);
                let cleaned = if let Ok(url) = parsed {
                    cleaner.clean_url(&url)
                } else {
                    None
                };

                if let Some(clean) = cleaned {
                    #[cfg(feature = "desktop-notifications")]
                    {
                        use notify_rust::Notification;

                        let text = format!("Cleaned link: {}", clean);
                        let _err = Notification::new()
                            .summary("ClipClean")
                            .body(&text)
                            .timeout(5000)
                            .show();
                    }

                    println!("Cleaned: {}", clean);

                    let _ = ctx.set_contents(clean);
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
