fn main() {
    // only run if target os is windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    // only build the resource for release builds
    // as calling rc.exe might be slow
    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = tauri_winres::WindowsResource::new();

        res.set_icon("icon.ico")
            // can't use winapi crate constants for cross compiling
            // MAKELANGID(LANG_ENGLISH, SUBLANG_ENGLISH_US )
            .set_language(0x0409)
            .set_manifest_file("manifest.xml");
        if let Err(e) = res.compile() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
