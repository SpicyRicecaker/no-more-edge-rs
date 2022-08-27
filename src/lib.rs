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
use winreg::enums::{HKEY_CLASSES_ROOT, HKEY_CURRENT_USER};
use winreg::RegKey;

use regex::Regex;

use std::time::Instant;

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

        let search_arg = match method {
            Method::Search => {
                let search_string = "https://google.com/search?q=";
                format!("{}{}", search_string, captured)
            }
            Method::Url => captured.to_string(),
        };

        time(open_path, &search_arg);
        time(open_registry, &search_arg);

        // println!("{}", captured);
        // pause()
    } else {
        println!("not implemented `{}`", url_argument);
        // pause()
    }
}

/// Times the amount of time it takes to run a function
pub fn time(function: fn(&str), argument: &str) {
    let instant = Instant::now();
    function(argument);
    dbg!(instant.elapsed());
}

/// Opens link using the default browser, with link directly to executable
pub fn open_path(argument: &str) {
    let mut browser = Command::new(r#"C:\Program Files\Firefox Developer Edition\firefox.exe"#);
    browser
        .arg("--new-tab")
        .arg(argument)
        .spawn()
        .expect("ERROR PATH");
}

pub fn open_registry(argument: &str) {
    // gets the default registry by traversing the registry twice
    // code inspired by https://stackoverflow.com/a/68292700/11742422
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let user_choice = hkcu
        .open_subkey(
            r#"SOFTWARE\Microsoft\Windows\Shell\Associations\UrlAssociations\http\UserChoice"#,
        )
        .unwrap();
    let prog_id: String = user_choice.get_value("ProgId").unwrap();
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    let command = hkcr
        .open_subkey(format!(r#"{prog_id}\shell\open\command"#))
        .unwrap();

    let complex_path: String = command.get_value("").unwrap();

    let browser = Regex::new(r#""([^"]*)""#).unwrap();
    let browser = browser.captures(&complex_path).unwrap()[1].to_string();

    let mut browser = Command::new(browser);
    browser
        .arg("--new-tab")
        .arg(argument)
        .spawn()
        .expect("ERROR PATH");
}

// dbg
pub fn pause() {
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
