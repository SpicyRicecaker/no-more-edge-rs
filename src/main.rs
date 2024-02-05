// remove console, because that'd be annoying when we're just trying to start a new tab
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use no_more_edge_rs::run;

fn main() {
    let args = std::env::args();

    // the arguments given to the program by windows are as follows:
    // 1. the path to the rust program itself
    // 2. the path to the edge program itself
    // 3. --single-argument
    // 4. long search url

    // we don't need arguments 1-3, so we skip through them
    let mut args = args.skip(3);

    // long search string we need to replace
    if let Some(arg) = args.next() {
        run(&arg);
    };
}