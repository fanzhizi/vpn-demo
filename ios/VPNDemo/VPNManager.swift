import Foundation
import NetworkExtension

/// Manages the VPN tunnel lifecycle using NETunnelProviderManager.
class VPNManager {

    static let shared = VPNManager()

    private var manager: NETunnelProviderManager?

    private init() {}

    /// Load or create the VPN configuration.
    func loadConfiguration(completion: @escaping (Error?) -> Void) {
        NETunnelProviderManager.loadAllFromPreferences { [weak self] managers, error in
            if let error = error {
                completion(error)
                return
            }

            // Use existing or create new
            self?.manager = managers?.first ?? NETunnelProviderManager()
            completion(nil)
        }
    }

    /// Configure and save the VPN profile.
    func configureVPN(socks5Address: String, completion: @escaping (Error?) -> Void) {
        guard let manager = manager else {
            completion(NSError(domain: "VPNManager", code: -1,
                             userInfo: [NSLocalizedDescriptionKey: "Manager not loaded"]))
            return
        }

        let proto = NETunnelProviderProtocol()
        // Bundle ID of the Network Extension target
        proto.providerBundleIdentifier = "com.vpndemo.app.tunnel"
        proto.serverAddress = socks5Address
        proto.providerConfiguration = [
            "socks5_address": socks5Address
        ]

        manager.protocolConfiguration = proto
        manager.localizedDescription = "VPN Demo"
        manager.isEnabled = true

        manager.saveToPreferences { error in
            if let error = error {
                completion(error)
                return
            }
            // Reload after save
            manager.loadFromPreferences { error in
                completion(error)
            }
        }
    }

    /// Start the VPN tunnel.
    func startVPN(socks5Address: String) throws {
        guard let manager = manager else {
            throw NSError(domain: "VPNManager", code: -1,
                         userInfo: [NSLocalizedDescriptionKey: "Manager not loaded"])
        }

        let session = manager.connection as! NETunnelProviderSession

        let options: [String: NSObject] = [
            "socks5_address": socks5Address as NSObject
        ]

        try session.startVPNTunnel(options: options)
    }

    /// Stop the VPN tunnel.
    func stopVPN() {
        guard let manager = manager else { return }
        manager.connection.stopVPNTunnel()
    }

    /// Get current VPN status.
    var status: NEVPNStatus {
        return manager?.connection.status ?? .invalid
    }
}
