#[cfg(not(feature = "std"))]
fn main() {
    eprintln!("only in feature = std");
    std::process::exit(1)
}

#[cfg(feature = "std")]
fn main() {
    use getopts_macro::{getopts_options, simple_parse};

    let options = getopts_options! {
        -h, --help          "show help message";
    };
    let matches = simple_parse(&options, "", 1, "<FILE..>");
    println!("files: {:?}", matches.free)
}
