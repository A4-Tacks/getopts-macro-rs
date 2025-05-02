Using macros to concisely define the getopts

```rust
use getopts_macro::getopts_options;

let _options = getopts_options! {
    -z --zero           "...";
    -v --verbose*       "...";
    -T --threads=NUM    "...";
    -i --ignore*=FILE   "...";
    -h --help*          "...";
    -V --version*       "...";
};
```

Expand to:

```rust
let _options = {
    let mut options = getopts::Options::new();
    options.optflag("z", "zero", "...");
    options.optflagmulti("v", "verbose", "...");
    options.optopt("T", "threads", "...", "NUM");
    options.optmulti("i", "ignore", "...", "FILE");
    options.optflagmulti("h", "help", "...");
    options.optflagmulti("V", "version", "...");
    options
};
```
