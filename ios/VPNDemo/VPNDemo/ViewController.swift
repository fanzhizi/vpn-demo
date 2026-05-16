import UIKit
import NetworkExtension

class ViewController: UIViewController {

    private let addressField = UITextField()
    private let connectButton = UIButton(type: .system)
    private let statusLabel = UILabel()
    private let statusDot = UIView()
    private let logLabel = UILabel()

    private var isConnected = false

    override func viewDidLoad() {
        super.viewDidLoad()
        setupUI()
        setupVPN()
        observeVPNStatus()
    }

    private func setupUI() {
        view.backgroundColor = .systemBackground

        let titleLabel = UILabel()
        titleLabel.text = "iOS VPN Demo"
        titleLabel.font = .systemFont(ofSize: 24, weight: .bold)
        titleLabel.textAlignment = .center

        addressField.placeholder = "SOCKS5 address (e.g. 1.2.3.4:1080)"
        addressField.text = "192.168.31.209:1080"
        addressField.borderStyle = .roundedRect
        addressField.autocapitalizationType = .none
        addressField.autocorrectionType = .no
        addressField.keyboardType = .numbersAndPunctuation

        statusDot.backgroundColor = .systemRed
        statusDot.layer.cornerRadius = 6
        statusDot.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            statusDot.widthAnchor.constraint(equalToConstant: 12),
            statusDot.heightAnchor.constraint(equalToConstant: 12),
        ])

        statusLabel.text = "Disconnected"
        statusLabel.font = .systemFont(ofSize: 14)

        let statusStack = UIStackView(arrangedSubviews: [statusDot, statusLabel])
        statusStack.axis = .horizontal
        statusStack.spacing = 8
        statusStack.alignment = .center

        connectButton.setTitle("Connect VPN", for: .normal)
        connectButton.titleLabel?.font = .systemFont(ofSize: 18, weight: .semibold)
        connectButton.addTarget(self, action: #selector(toggleVPN), for: .touchUpInside)

        let testButton = UIButton(type: .system)
        testButton.setTitle("Test SOCKS5", for: .normal)
        testButton.titleLabel?.font = .systemFont(ofSize: 16, weight: .medium)
        testButton.addTarget(self, action: #selector(testSocks5), for: .touchUpInside)

        logLabel.text = ""
        logLabel.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        logLabel.textColor = .secondaryLabel
        logLabel.numberOfLines = 0
        logLabel.textAlignment = .center

        let stack = UIStackView(arrangedSubviews: [titleLabel, addressField, statusStack, connectButton, testButton, logLabel])
        stack.axis = .vertical
        stack.spacing = 16
        stack.alignment = .center
        stack.translatesAutoresizingMaskIntoConstraints = false

        view.addSubview(stack)
        NSLayoutConstraint.activate([
            stack.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            stack.centerYAnchor.constraint(equalTo: view.centerYAnchor),
            addressField.widthAnchor.constraint(equalTo: view.widthAnchor, multiplier: 0.8),
            logLabel.widthAnchor.constraint(equalTo: view.widthAnchor, multiplier: 0.9),
        ])
    }

    private func appendLog(_ msg: String) {
        DispatchQueue.main.async {
            let current = self.logLabel.text ?? ""
            let lines = current.split(separator: "\n").suffix(5)
            self.logLabel.text = (lines + [Substring(msg)]).joined(separator: "\n")
        }
        print("[VPNDemo] \(msg)")
    }

    private func setupVPN() {
        appendLog("Loading VPN config...")
        VPNManager.shared.loadConfiguration { [weak self] error in
            if let error = error {
                self?.appendLog("Load error: \(error.localizedDescription)")
            } else {
                self?.appendLog("VPN config loaded")
            }
        }
    }

    private func observeVPNStatus() {
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(vpnStatusChanged),
            name: .NEVPNStatusDidChange,
            object: nil
        )
    }

    @objc private func vpnStatusChanged() {
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }
            let status = VPNManager.shared.status
            self.appendLog("VPN status: \(status.rawValue)")
            switch status {
            case .connected:
                self.statusDot.backgroundColor = .systemGreen
                self.statusLabel.text = "Connected"
                self.connectButton.setTitle("Disconnect", for: .normal)
                self.addressField.isEnabled = false
                self.isConnected = true
            case .connecting:
                self.statusDot.backgroundColor = .systemOrange
                self.statusLabel.text = "Connecting..."
            case .disconnecting:
                self.statusDot.backgroundColor = .systemOrange
                self.statusLabel.text = "Disconnecting..."
            default:
                self.statusDot.backgroundColor = .systemRed
                self.statusLabel.text = "Disconnected"
                self.connectButton.setTitle("Connect VPN", for: .normal)
                self.addressField.isEnabled = true
                self.isConnected = false
            }
        }
    }

    @objc private func toggleVPN() {
        if isConnected {
            appendLog("Stopping VPN...")
            VPNManager.shared.stopVPN()
        } else {
            let address = addressField.text ?? "192.168.31.209:1080"
            appendLog("Configuring VPN with \(address)...")
            VPNManager.shared.configureVPN(socks5Address: address) { [weak self] error in
                if let error = error {
                    self?.appendLog("Config error: \(error.localizedDescription)")
                    return
                }
                self?.appendLog("Config saved, starting tunnel...")
                do {
                    try VPNManager.shared.startVPN(socks5Address: address)
                    self?.appendLog("startVPNTunnel called OK")
                } catch {
                    self?.appendLog("Start error: \(error.localizedDescription)")
                }
            }
        }
    }

    @objc private func testSocks5() {
        let address = addressField.text ?? ""
        appendLog("Testing SOCKS5: \(address)...")

        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            var buf = [UInt8](repeating: 0, count: 2048)
            let result = address.withCString { ptr in
                tunnel_test_socks5(ptr, &buf, buf.count)
            }
            let msg = String(cString: buf.map { CChar(bitPattern: $0) })
            DispatchQueue.main.async {
                self?.appendLog("Test result(\(result)): \(msg)")
            }
        }
    }
}
