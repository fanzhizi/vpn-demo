import UIKit
import NetworkExtension

/// Simple ViewController as a fallback if Slint is not available.
/// In production, the Slint UI would be the primary interface.
class ViewController: UIViewController {

    private let addressField = UITextField()
    private let connectButton = UIButton(type: .system)
    private let statusLabel = UILabel()
    private let statusDot = UIView()

    private var isConnected = false

    override func viewDidLoad() {
        super.viewDidLoad()
        setupUI()
        setupVPN()
        observeVPNStatus()
    }

    private func setupUI() {
        view.backgroundColor = .systemBackground
        title = "VPN Demo"

        // Title
        let titleLabel = UILabel()
        titleLabel.text = "iOS VPN Demo"
        titleLabel.font = .systemFont(ofSize: 24, weight: .bold)
        titleLabel.textAlignment = .center

        // Address input
        addressField.placeholder = "SOCKS5 address (e.g. 1.2.3.4:1080)"
        addressField.text = "127.0.0.1:1080"
        addressField.borderStyle = .roundedRect
        addressField.autocapitalizationType = .none
        addressField.autocorrectionType = .no
        addressField.keyboardType = .numbersAndPunctuation

        // Status
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

        // Button
        connectButton.setTitle("Connect VPN", for: .normal)
        connectButton.titleLabel?.font = .systemFont(ofSize: 18, weight: .semibold)
        connectButton.addTarget(self, action: #selector(toggleVPN), for: .touchUpInside)

        // Layout
        let stack = UIStackView(arrangedSubviews: [titleLabel, addressField, statusStack, connectButton])
        stack.axis = .vertical
        stack.spacing = 20
        stack.alignment = .center
        stack.translatesAutoresizingMaskIntoConstraints = false

        view.addSubview(stack)
        NSLayoutConstraint.activate([
            stack.centerXAnchor.constraint(equalTo: view.centerXAnchor),
            stack.centerYAnchor.constraint(equalTo: view.centerYAnchor),
            addressField.widthAnchor.constraint(equalTo: view.widthAnchor, multiplier: 0.8),
        ])
    }

    private func setupVPN() {
        VPNManager.shared.loadConfiguration { error in
            if let error = error {
                print("Failed to load VPN config: \(error)")
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
            VPNManager.shared.stopVPN()
        } else {
            let address = addressField.text ?? "127.0.0.1:1080"
            VPNManager.shared.configureVPN(socks5Address: address) { [weak self] error in
                if let error = error {
                    print("Config error: \(error)")
                    return
                }
                do {
                    try VPNManager.shared.startVPN(socks5Address: address)
                } catch {
                    print("Start error: \(error)")
                }
            }
        }
    }
}
