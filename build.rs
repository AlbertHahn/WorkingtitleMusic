// extern crate embed_resource;
use fs_extra::{copy_items, dir::CopyOptions};

fn main() {
    // let target = std::env::var("TARGET").unwrap();
    // if target.contains("windows") || target.contains("linux") {
    //     embed_resource::compile("src/baked_assets/icon.rc", embed_resource::NONE);
    // }

    // FMOD
    // println!("cargo:rustc-env=RUSTFLAGS=-C runpath=./lib");
    // println!("cargo:rustc-link-search=native={}", "lib");

    // println!("target_os = {}", std::env::var("CARGO_CFG_TARGET_OS").unwrap());

    let mut target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let fmod_api_loc: Vec<String>;
    #[cfg(target_os = "linux")]
    {
        // Account for the naming mismatch between FMOD library folders and the target architecture
        if target_arch == "aarch64" {
            target_arch = "arm64".into();
        }
    }

    // // not needed, simply placed the windows files into the "more correct" x86_64 directory
    // #[cfg(target_family = "windows")]{
    //     target_arch = "x64".into();
    // }

    fmod_api_loc = vec![
        format!("./fmod/api/core/lib/{target_arch}"),
        format!("./fmod/api/studio/lib/{target_arch}"),
    ];
    for loc in fmod_api_loc {
        println!("cargo:rustc-link-search={loc}");
        println!("cargo:rustc-env=LD_RUN_PATH={loc}");
        
        #[cfg(debug_assertions)]
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../.{loc}");
    }


    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");
}
