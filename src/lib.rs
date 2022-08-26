//! # no-more-edge-rs
//!
//! Replaces edge calls to your default browser
//!
//! ## Installation
//!
//! ```shell
//! # install the program itself
//! cargo install no-more-edge-rs
//! # replace the single registry script
//! no-more-edge-rs install
//! ```
//!
//! ## Uninstall
//!
//! ```shell
//! no-more-edge-rs uninstall
//! cargo uninstall no-more-edge-rs
//! ```
//!
//! ## CLI Options
//!
//! Set the default search engine (defaults to google)
//!
//! ```shell
//! no-more-edge-rs set-engine https://google.com?q=
//! ```

use percent_encoding::percent_decode_str;
use std::io::stdin;
use std::process::Command;

use regex::Regex;

enum Method {
    Url,
    Search,
}

struct CaptureGetter {
    regex: Regex,
    method: Method,
}

impl CaptureGetter {
    fn new(regex: Regex, method: Method) -> Self {
        Self { regex, method }
    }
}

pub fn run(url_argument: String) {
    let capture_getters = [
        CaptureGetter::new(
            Regex::new("https%3A%2F%2Fwww.bing.com%2Fsearch%3Fq%3D(.*)%26").unwrap(),
            Method::Search,
        ),
        CaptureGetter::new(
            Regex::new("https%3A%2F%2Fwww.bing.com%2FWS%2Fredirect%2F%3Fq%3D(.*)%26").unwrap(),
            Method::Url,
        ),
        // regex would be useful here for optional matchin
        CaptureGetter::new(
            Regex::new("&url=http(?:s)*%3A%2F%2F(.*)%2F").unwrap(),
            Method::Url,
        ),
    ];

    let captured = capture_getters.into_iter().find_map(|capture_getter| {
        capture_getter
            .regex
            .captures(&url_argument)
            .map(|t| (t[1].to_string(), capture_getter.method))
    });

    if let Some((captured, method)) = captured {
        let captured = percent_decode_str(&captured)
            .decode_utf8()
            .expect("error unescaping");

        // let mut ff = Command::new(r#"C:\Program Files\Firefox Developer Edition\firefox.exe"#);
        let mut ff = Command::new(r#"start"#);
        match method {
            Method::Search => {
                let search_string = "https://google.com/search?q=";
                ff.arg("--new-tab")
                    .arg([search_string, &captured].join(""))
                    .spawn()
                    .expect("ERROR PATH");
            }
            Method::Url => {
                ff.arg("--new-tab")
                    .arg(captured.as_ref())
                    .spawn()
                    .expect("ERROR PATH");
            }
        }

        // println!("{}", captured);
        // pause()
    } else {
        println!("not implemented `{}`", url_argument);

        pause()
    }
}

// dbg
fn pause() {
    println!("Press enter to continue...");
    let mut buffer = String::new();
    loop {
        stdin().read_line(&mut buffer).unwrap();

        if buffer.trim().is_empty() {
            break;
        }

        buffer.clear();
    }
}
