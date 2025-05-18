use std::{path::Path, thread};

use ctor::ctor;
use windows::{core::w, Win32::System::LibraryLoader::LoadLibraryW};

#[ctor]
fn entry() {
    thread::spawn(|| unsafe {
        inject_loader();
    });
}

fn inject_loader() {
	if Path::new("starloader.dll").exists() {
        unsafe {
            LoadLibraryW(w!("starloader.dll")).unwrap();
        }
    }
}