//!
//! Replaces calls to microsoft edge with calls to your default browser on
//! windows. Inspired by the c# project
//! [NoMoreEdge](https://github.com/HarshalKudale/NoMoreEdge).
//!
//! ## Installation
//! 
//! Simply download and run the `.msi` installer in releases. 
//! 
//! ### Uninstallation
//! 
//! Uninstall the program as you would a regular windows program in control
//! panel. This program only registers a single registry key, so uninstallation
//! is just a matter of deleting that key.
//! 
//! ## Building Manually
//! 
//! This project uses [cargo-wix](https://github.com/volks73/cargo-wix) to build the `.msi` installer for the app and write the necessary registry key. Install it via 
//! 
//! ```shell
//! cargo install cargo-wix
//! ```
//! 
//! Then simply run 
//! 
//! ```shell
//! cargo wix
//! ```
//! 
//! Then to install the program, run the `.msi` file in the `./wix` folder.

use percent_encoding::percent_decode_str;
use std::io::stdin;
use std::process::Command;
use winreg::enums::{HKEY_CLASSES_ROOT, HKEY_CURRENT_USER};
use winreg::RegKey;

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

/// This function takes in a search string, (which windows originally intends to
/// pass into edge.exe), and redirects it to the default browser
pub fn run(url_argument: String) {
    // The known patterns of search queries which are given to edge
    let capture_getters = [
        CaptureGetter::new(
            Regex::new("https%3A%2F%2Fwww.bing.com%2Fsearch%3Fq%3D(.*)%26").unwrap(),
            Method::Search,
        ),
        CaptureGetter::new(
            Regex::new("https%3A%2F%2Fwww.bing.com%2FWS%2Fredirect%2F%3Fq%3D(.*)%26").unwrap(),
            Method::Url,
        ),
        CaptureGetter::new(
            Regex::new("&url=http(?:s)*%3A%2F%2F(.*)").unwrap(),
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

        open_registry(&search_arg);
    } else {
        println!("not implemented `{}`", url_argument);
    }
}

#[inline(always)]
pub fn open_registry(argument: &str) {
    // gets the default registry by traversing the registry twice
    // code inspired by https://stackoverflow.com/a/68292700/11742422
    let user_choice = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(
            r"SOFTWARE\Microsoft\Windows\Shell\Associations\UrlAssociations\http\UserChoice",
        )
        .unwrap();
    let prog_id: String = user_choice.get_value("ProgId").unwrap();

    let command = RegKey::predef(HKEY_CLASSES_ROOT)
        .open_subkey(format!(r#"{prog_id}\shell\open\command"#))
        .unwrap();

    let complex_path: String = command.get_value("").unwrap();

    let browser = Regex::new(r#""([^"]*)""#).unwrap();
    let browser = browser.captures(&complex_path).unwrap()[1].to_string();

    Command::new(browser)
        .arg("--new-tab")
        .arg(argument)
        .spawn()
        .expect("ERROR PATH");
}

/// A debug function.
/// When called, waits for input into the console before continuing
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
