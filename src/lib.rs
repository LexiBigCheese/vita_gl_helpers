use std::ffi::CString;

pub mod attribute;
pub mod buffer;
pub mod draw;
pub mod errors;
pub mod program;
pub mod shader;
pub mod texture;
pub mod uniforms;

// #[link(name = "vitaGL", kind = "static")]
// #[link(name = "vitashark", kind = "static")]
// #[link(name = "SceShaccCg_stub", kind = "static")]
// #[link(name = "mathneon", kind = "static")]
// #[link(name = "SceShaccCgExt", kind = "static")]
// #[link(name = "taihen_stub", kind = "static")]
// #[link(name = "SceKernelDmacMgr_stub", kind = "static")]
// #[link(name = "SceIme_stub", kind = "static")]
// #[link(name = "SceGxm_stub", kind = "static")]
// #[link(name = "SceDisplay_stub", kind = "static")]
// #[link(name = "SceAppMgr_stub", kind = "static")]
// #[link(name = "SceCommonDialog_stub", kind = "static")]
unsafe extern "C" {
    pub fn vglSwapBuffers(has_commondialog: u8);
    pub fn vglSetupRuntimeShaderCompiler(
        opt_level: i32,
        use_fastmath: i32,
        use_fastprecision: i32,
        use_fastint: i32,
    );
    pub fn vglInitExtended(
        legacy_pool_size: i32,
        width: i32,
        height: i32,
        ram_threshold: i32,
        msaa: u32,
    ) -> u8;
    pub fn vglGetProcAddress(name: *const u8) -> *const u8;
}

pub fn swap_buffers() {
    unsafe {
        vglSwapBuffers(0);
    }
}

pub struct RuntimeShaderCompilerSettings {
    pub opt_level: i32,
    pub use_fastmath: i32,
    pub use_fastprecision: i32,
    pub use_fastint: i32,
}

impl Default for RuntimeShaderCompilerSettings {
    fn default() -> Self {
        RuntimeShaderCompilerSettings {
            opt_level: 2,
            use_fastmath: 1,
            use_fastprecision: 0,
            use_fastint: 1,
        }
    }
}

pub struct VglInitSettings {
    pub legacy_pool_size: i32,
    pub ram_threshold: i32,
    pub msaa: u32,
}

impl Default for VglInitSettings {
    fn default() -> Self {
        VglInitSettings {
            legacy_pool_size: 0,
            ram_threshold: 65 * 1024 * 1024,
            msaa: 0,
        }
    }
}

pub fn initialise_extended(rscs: RuntimeShaderCompilerSettings, vis: VglInitSettings) {
    unsafe {
        vglSetupRuntimeShaderCompiler(
            rscs.opt_level,
            rscs.use_fastmath,
            rscs.use_fastprecision,
            rscs.use_fastint,
        );
        vglInitExtended(vis.legacy_pool_size, 960, 544, vis.ram_threshold, vis.msaa);
    }
    gl::load_with(|name| {
        let name = CString::new(name).unwrap();
        unsafe { vglGetProcAddress(name.as_ptr() as _) as _ }
    });
}

pub fn initialise_default() {
    initialise_extended(Default::default(), Default::default());
}
