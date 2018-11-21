extern crate clipboard;
extern crate url;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use url::*;
use clipclean::*;


fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    loop {
        let string = ctx.get_contents();
        if let Ok(s) = string {
            let parsed = Url::parse(&s);
            let cleaned = if let Ok(url) = parsed {
                clean_url(&url)
            } else {
                None
            };

            if let Some(clean) = cleaned {
                println!("cleaned: {}", clean);
                let _ = ctx.set_contents(clean);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}


