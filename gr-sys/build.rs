use std::env;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Searcher(Option<PathBuf>);

impl Searcher {
    pub fn new() -> Searcher {
        println!("Searching GR:");
        Searcher(None)
    }

    pub fn consider(&mut self, p: impl Into<PathBuf>) -> &mut Self {
        let None = &self.0 else {
            return self;
        };
        const EXTENSION: &'static str = if cfg!(windows) {
            "lib"
        } else if cfg!(target_vendor = "apple") {
            "dylib"
        } else {
            "so"
        };
        let mut p = p.into();
        p.push("libGR");
        p.set_extension(EXTENSION);
        println!("Trying: {}", p.display());
        if p.is_file() {
            p.pop();
            self.0 = Some(p);
        }
        self
    }

    pub fn result(self) -> Option<PathBuf> {
        let out = match self.0 {
            Some(_) => "Found",
            None => "Failed",
        };
        println!("{out}");
        self.0
    }
}

fn main() {
    let mut searcher = Searcher::new();
    if let Some(lib_dir) = env::var_os("GRLIB") {
        searcher.consider(lib_dir);
    };
    if !cfg!(windows) {
        searcher
            .consider("~/gr/lib/")
            .consider("/usr/local/gr/lib/")
            .consider("/usr/gr/lib/");
    }
    if let Some(mut lib_dir) = searcher.result() {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        if cfg!(windows) {
            lib_dir.pop();
            lib_dir.push("bin");
        }
        println!("cargo:lib_dir={}", lib_dir.display());
    };
    let names = ["GKS", "GR"].into_iter();
    #[cfg(windows)]
    let names = names.map(|name| String::from("lib") + name);
    names.for_each(|name| println!("cargo:rustc-link-lib=dylib={name}"));
    #[cfg(feature = "bindgen")]
    {
        use bindgen::callbacks::{IntKind, ParseCallbacks};
        #[derive(Debug)]
        struct IntCallback;
        impl ParseCallbacks for IntCallback {
            fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind> {
                Some(IntKind::Int)
            }
        }
        println!("Generating bindings:");
        let mut out_path = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set"));
        let mut raw_path = PathBuf::from("raw");
        out_path.push("dummy");
        raw_path.push("dummy");
        let entries = std::fs::read_dir("header").unwrap(); // should never fail
        for header in entries {
            let header = header
                .unwrap() // should never fail
                .path();
            let name = header.file_name().unwrap(); // should also never fail
            out_path.set_file_name(name);
            out_path.set_extension("rs");
            raw_path.set_file_name(name);
            raw_path.set_extension("rs");
            println!("{} -> {}", header.display(), out_path.display());
            let mut builder = bindgen::builder()
                .header(header.to_str().unwrap())
                .use_core()
                .layout_tests(false)
                .parse_callbacks(Box::new(IntCallback))
                .blocklist_type("size_t");
            if raw_path.is_file() {
                builder = builder.raw_line(std::fs::read_to_string(&raw_path).unwrap());
            }
            builder
                .generate()
                .unwrap()
                .write_to_file(out_path.clone())
                .unwrap();
        }
    }
    if !cfg!(feature = "bindgen") {
        println!("Copying bindings:");
        let mut out_path = env::var_os("OUT_DIR")
            .map(PathBuf::from)
            .expect("OUT_DIR not set");
        out_path.push("dummy");
        let entries = std::fs::read_dir("bindings").unwrap(); // should never fail
        for bindings in entries {
            let binding_path = bindings
                .unwrap() // should never fail
                .path();
            let name = binding_path.file_name().unwrap(); // should also never fail
            out_path.set_file_name(name);
            println!("{} -> {}", binding_path.display(), out_path.display());
            std::fs::copy(binding_path, &out_path).unwrap();
        }
    }
}
