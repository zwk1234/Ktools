fn main() {
    cc::Build::new()
        .flag("-D_7ZIP_ST")
        .file("7zSDK/C/Alloc.c")
        .file("7zSDK/C/LzFind.c")
        .file("7zSDK/C/LzmaDec.c")
        .file("7zSDK/C/LzmaEnc.c")
        .file("7zSDK/C/7zFile.c")
        .file("7zSDK/C/7zStream.c")
        .file("7zSDK/LzmaUtil.c")
        .compile("lib7z");

    // let bindings = bindgen::Builder::default()
    //     .header("LzmaUtil.h")
    //     .whitelist_function("main2")
    //     .generate()
    //     .expect("unable to generate hello bindings");
}