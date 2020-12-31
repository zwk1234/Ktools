use std::{env, path::PathBuf};

fn main() {
    // cc::Build::new()
    //     .flag("-D_7ZIP_ST")
    //     .file("7zSDK/C/Alloc.c")
    //     .file("7zSDK/C/LzFind.c")
    //     .file("7zSDK/C/LzmaDec.c")
    //     .file("7zSDK/C/LzmaEnc.c")
    //     .file("7zSDK/C/7zFile.c")
    //     .file("7zSDK/C/7zStream.c")
    //     .file("7zSDK/LzmaUtil.c")
    //     .compile("lib7z");

    // let bindings = bindgen::Builder::default()
    //     .header("LzmaUtil.h")
    //     .whitelist_function("main2")
    //     .generate()
    //     .expect("unable to generate hello bindings");

    let library_name = "bit7z64";

    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let library_dir = dunce::canonicalize(root.join("bit7z/bin/x64")).unwrap();

    println!("cargo:rustc-link-search=native={}", env::join_paths(&[library_dir]).unwrap().to_str().unwrap());
    println!("cargo:rustc-link-lib=static={}", library_name);
    println!("cargo:rustc-flags=-l dylib=oleaut32");
}