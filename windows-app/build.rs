fn main() {
    slint_build::compile("../vpn-app/ui/main.slint").unwrap();

    // Windows: embed manifest to request admin privileges (UAC)
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("app.manifest");
        res.set("FileDescription", "VPN Demo");
        res.set("ProductName", "VPN Demo");
        res.compile().unwrap();
    }
}
