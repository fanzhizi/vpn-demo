slint::include_modules!();

fn main() {
    let window = MainWindow::new().unwrap();

    let window_weak = window.as_weak();
    window.on_connect_clicked(move |addr| {
        let window = window_weak.unwrap();
        let addr_str = addr.to_string();

        if addr_str.parse::<std::net::SocketAddr>().is_err() {
            window.set_status_text("Invalid address format".into());
            return;
        }

        window.set_connected(true);
        window.set_status_text(slint::format!("Connected to {}", addr_str));
        window.set_log_text(slint::format!("VPN connected via {}", addr_str));
    });

    let window_weak = window.as_weak();
    window.on_disconnect_clicked(move || {
        let window = window_weak.unwrap();
        window.set_connected(false);
        window.set_status_text("Disconnected".into());
        window.set_log_text("VPN disconnected".into());
    });

    let window_weak = window.as_weak();
    window.on_test_clicked(move |addr| {
        let window = window_weak.unwrap();
        window.set_log_text(slint::format!("Testing SOCKS5: {}...", addr));
    });

    window.run().unwrap();
}
