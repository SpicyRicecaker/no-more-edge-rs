use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};
use std::io::stdin;
use std::process::Command;

struct StrStateMachine {
    idx: usize,
    start: Option<usize>,
    end: Option<usize>,
    first: Vec<char>,
}

impl StrStateMachine {
    fn new(state: &str) -> Self {
        Self {
            idx: 0,
            start: None,
            end: None,
            first: state.chars().collect(),
        }
    }

    fn advance(&mut self, c: char, index: usize) -> bool {
        if self.first[self.idx] == c {
            if self.start.is_none() {
                self.start = Some(index);
            }
            self.idx += 1;
            if self.idx == self.first.len() {
                self.end = Some(index);
                false
            } else {
                true
            }
        } else {
            self.idx = 0;
            self.start = None;
            true
        }
    }

    fn is_complete(&self) -> bool {
        self.end.is_some()
    }
}

struct Betweener {
    first: StrStateMachine,
    second: StrStateMachine,
}

impl Betweener {
    fn new(first: &str, second: &str) -> Self {
        Self {
            first: StrStateMachine::new(first),
            second: StrStateMachine::new(second),
        }
    }

    fn between(mut self, string: &str) -> Option<&str> {
        for (i, c) in string.char_indices() {
            if !self.first.is_complete() {
                self.first.advance(c, i);
            } else if !self.second.is_complete() {
                self.second.advance(c, i);
            } else {
                break;
            }
        }

        // could break
        if let Some(begin) = self.first.end && let Some (end) = self.second.start {
            Some(&string[begin+1..end])
        } else {
            None
        }
    }
}

enum Method {
    Search,
    Url,
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
    if let Some(sent_url) = args.next() {
        let urls = [
            (
                "https%3A%2F%2Fwww.bing.com%2Fsearch%3Fq%3D",
                "%26",
                Method::Search,
            ),
            (
                "https%3A%2F%2Fwww.bing.com%2FWS%2Fredirect%2F%3Fq%3D",
                "%26",
                Method::Url,
            ),
            // regex would be useful here for optional matchin
            ("&url=http%3A%2F%2F", "%2F", Method::Url),
            ("&url=https%3A%2F%2F", "%2F", Method::Url),
        ];

        let s = urls.iter().find_map(|(first, second, method)| {
            let s = sent_url.as_str().get_between(first, second);
            s.map(|s| (s, method))
        });

        if let Some((s, method)) = s {
            let s = percent_decode_str(s)
                .decode_utf8()
                .expect("error unescaping");
            // let s = ;

            println!("{s}");

            let mut ff = Command::new(r#"C:\Program Files\Firefox Developer Edition\firefox.exe"#);
            match method {
                Method::Search => {
                    let search_string = "https://google.com/search?q=";
                    ff.arg("--new-tab")
                        .arg([search_string, &s].join(""))
                        .spawn()
                        .expect("ERROR PATH");
                }
                Method::Url => {
                    ff.arg("--new-tab")
                        .arg(s.as_ref())
                        .spawn()
                        .expect("ERROR PATH");
                }
            }
            pause()
        } else {
            println!("not implemented `{}`", sent_url);
            pause()
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

trait Betweenable {
    fn get_between(&self, first: &str, second: &str) -> Option<&str>;
}

impl Betweenable for str {
    fn get_between(&self, first: &str, second: &str) -> Option<&str> {
        let betweener = Betweener::new(first, second);
        betweener.between(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct FirstMiddleLast {
        first: String,
        middle: String,
        last: String,
    }

    impl FirstMiddleLast {
        fn new(first: &str, second: &str, third: &str) -> Self {
            FirstMiddleLast {
                first: String::from(first),
                middle: String::from(second),
                last: String::from(third),
            }
        }
    }

    #[test]
    fn testbetween() {
        let fmls = [
            FirstMiddleLast::new("fruit", "ILIKEPINEAPPLES", "pineapple"),
            FirstMiddleLast::new("A", "o", "B"),
        ];

        for fml in fmls {
            let full = [&fml.first, &fml.middle, &fml.last]
                .iter()
                .map(|&c| String::from(c))
                .collect::<Vec<String>>()
                .join("");

            let f = full.get_between(&fml.first, &fml.last);
            assert_eq!(f, Some(fml.middle.as_str()))
        }
    }
}
