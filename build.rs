fn main() {
    for i in [
        "vitaGL",
        "vitashark",
        "SceShaccCg_stub",
        "mathneon",
        "SceShaccCgExt",
        "taihen_stub",
        "SceKernelDmacMgr_stub",
        "SceIme_stub",
        "SceGxm_stub",
        "SceDisplay_stub",
        "SceAppMgr_stub",
        "SceCommonDialog_stub",
    ] {
        println!("cargo:rustc-link-lib={i}");
    }
}
