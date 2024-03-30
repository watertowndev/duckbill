///Initialize WinResource to enable pretty icons and details in the EXE
extern crate winresource;

pub fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("scadadrop.ico");
        res.compile().unwrap();
    }
}