fn main() {
    let mut windows = tauri_build::WindowsAttributes::new();
    // tauri-winres writes the icon's absolute path into the .rc file and escapes ' as \',
    // which rc.exe rejects (e.g. "C:\Jobert's Projects\..."). Build from a copy instead.
    if std::env::current_dir().unwrap().to_string_lossy().contains('\'') {
        let copy = std::env::temp_dir().join("nbsbuddy-icon.ico");
        std::fs::copy("icons/icon.ico", &copy).expect("copy icons/icon.ico");
        println!("cargo:rerun-if-changed=icons/icon.ico");
        windows = windows.window_icon_path(copy);
    }
    tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows)).expect("tauri-build failed");
}
