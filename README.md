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
