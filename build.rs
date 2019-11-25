use std::env;
use std::path::Path;

use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path =
        Path::new(&out_dir)
            .join("..").join("..").join("..");

    let mut options = CopyOptions::new();
    options.overwrite = true;

    copy("res", dest_path, &options).unwrap();
}
