use std::env;
use std::path::PathBuf;

fn main() {
    let Some(lib_dir) = env::var_os("DEP_GR_LIB_DIR") else {
        return;
    };
    let lib_dir = PathBuf::from(lib_dir);
    let lib_dir = lib_dir.display();
    println!("cargo:lib_dir={lib_dir}");
    if cfg!(windows) {
        let path = env::var_os("PATH")
            .map(PathBuf::from)
            .unwrap_or_default();
        let path = path.display();
        println!("cargo:rustc-env=PATH={lib_dir};{path}");
    } else {
        println!("cargo:rustc-link-arg-examples=-Wl,-rpath,{lib_dir}");
    }
}
