fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        static_vcruntime::metabuild();
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/Keychron_icon.ico");
        res.compile().unwrap();
    }
}
