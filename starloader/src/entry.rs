use std::{os::windows::ffi::OsStrExt, path::Path, thread};

use ctor::ctor;
use walkdir::WalkDir;
use windows::{
    Win32::{
        Foundation::FreeLibrary,
        System::LibraryLoader::{GetModuleHandleW, LoadLibraryW},
    },
    core::{PCWSTR, w},
};

#[ctor]
fn entry() {
    thread::spawn(|| unsafe {
        load_plugins();
    });
}

unsafe fn load_plugins() {
    unsafe {
        // Unload the dummy
        let handle = GetModuleHandleW(w!("astrolabe.dll")).unwrap();
        FreeLibrary(handle).unwrap();

        // Load the real module
        LoadLibraryW(w!(r"StarRail_Data\Plugins\x86_64\astrolabe.dll")).unwrap();
    }
    let plugins_dir = Path::new("plugins");
    if plugins_dir.exists() {
        for entry in WalkDir::new(plugins_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "dll" {
                    let plugin_path = path
                        .as_os_str()
                        .encode_wide()
                        .chain(Some(0))
                        .collect::<Vec<_>>();
                    unsafe {
                        LoadLibraryW(PCWSTR::from_raw(plugin_path.as_ptr())).unwrap();
                        log::info!(
                            "Loaded plugin {}",
                            path.file_stem().unwrap().to_str().unwrap()
                        );
                    }
                }
            }
        }
    } else {
        std::fs::create_dir_all(plugins_dir).unwrap();
    }
}
