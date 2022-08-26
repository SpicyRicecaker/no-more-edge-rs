#![windows_subsystem = "windows"]

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

fn main() {
    let mut args = std::env::args();

    // rust program itself
    args.next();
    // edge as an argument
    args.next();
    // single argument
    args.next();
    
    // long search string we need to replace
    if let Some(url_argument) = args.next() {
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
                Regex::new("&url=http(?:s)%3A%2F%2F(.*)%2F").unwrap(),
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

            let mut ff = Command::new(r#"C:\Program Files\Firefox Developer Edition\firefox.exe"#);
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
            // pause()
        } else {
            println!("not implemented `{}`", url_argument);
            // pause()
        }
    } else {
        panic!("search argument not provided");
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
