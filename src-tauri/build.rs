fn main() {
    #[cfg(windows)]
    {
        // Attach the requireAdministrator manifest on Windows builds.
        let windows = tauri_build::WindowsAttributes::new()
            .app_manifest(include_str!("app.manifest"));
        let attrs = tauri_build::Attributes::new().windows_attributes(windows);
        tauri_build::try_build(attrs).expect("failed to run tauri-build");
        return;
    }

    #[cfg(not(windows))]
    {
        tauri_build::build();
    }
}
