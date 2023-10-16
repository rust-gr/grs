use std::env;
use std::path::PathBuf;

fn main() {
    let lib_dir = env::var_os("DEP_GR_LIB_DIR")
        .map(PathBuf::from)
        .expect("gr-sys didn't output LIB_DIR");
    let lib_dir = lib_dir.display();
    println!("cargo:lib_dir={}", lib_dir);
    if cfg!(windows) {
        let path = env::var_os("PATH")
            .map(PathBuf::from)
            .expect("no PATH set!?");
        println!("cargo:rustc-env=PATH={};{}", lib_dir, path.display());
    } else {
        println!("cargo:rustc-link-arg-examples=-Wl,-rpath,{}", lib_dir);
    }
}
