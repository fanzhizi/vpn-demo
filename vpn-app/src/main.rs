slint::include_modules!();

fn main() {
    let window = MainWindow::new().unwrap();

    let window_weak = window.as_weak();
    window.on_connect_clicked(move |addr| {
        let window = window_weak.unwrap();
        let addr_str = addr.to_string();

        // Validate address format
        if addr_str.parse::<std::net::SocketAddr>().is_err() {
            window.set_status_text("Invalid address format".into());
            return;
        }

        // On iOS this would trigger NETunnelProviderManager to start the VPN
        // For now just update UI state
        window.set_connected(true);
        window.set_status_text(slint::format!("Connected to {}", addr_str));

        #[cfg(target_os = "ios")]
        {
            // Start VPN via NetworkExtension (called from Swift bridge)
            println!("Starting VPN tunnel to SOCKS5: {}", addr_str);
        }
    });

    let window_weak = window.as_weak();
    window.on_disconnect_clicked(move || {
        let window = window_weak.unwrap();
        window.set_connected(false);
        window.set_status_text("Disconnected".into());

        #[cfg(target_os = "ios")]
        {
            println!("Stopping VPN tunnel");
        }
    });

    window.run().unwrap();
}
