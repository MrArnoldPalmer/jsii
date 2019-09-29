use fs_extra::dir::{self, CopyOptions};
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut opts = CopyOptions::new();
    opts.overwrite = true;
    dir::copy("./node_modules/jsii-runtime/webpack", out_dir, &opts)
        .expect("Can't copy jsii-runtime to target");
}
