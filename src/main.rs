// #![windows_subsystem = "windows"]

use no_more_edge_rs::{run, pause};

// remove console, because that'd be annoying when we're just trying to start a new tab
fn main() {
    let args = std::env::args();
    dbg!(args.collect::<Vec<String>>());

    let mut args = std::env::args();
    
    // rust program itself, so we don't need this
    args.next();

    // the edge program itself is passed in by windows, which we don't need
    args.next();

    // `--single-argument` is also uneeded
    args.next();
    
    // long search string we need to replace
    if let Some(arg) = args.next() {
        // println!("{arg}");
        run(arg);
    };
    // pause()
}
