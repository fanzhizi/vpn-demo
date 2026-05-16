import Foundation
import NetworkExtension

class VPNManager {

    static let shared = VPNManager()

    private var manager: NETunnelProviderManager?

    private init() {}

    func loadConfiguration(completion: @escaping (Error?) -> Void) {
        NETunnelProviderManager.loadAllFromPreferences { [weak self] managers, error in
            if let error = error {
                completion(error)
                return
            }
            self?.manager = managers?.first ?? NETunnelProviderManager()
            completion(nil)
        }
    }

    /// Configure VPN tunnel. The serverAddress is just a display label for iOS,
    /// the actual SOCKS5 forwarding happens internally in the tunnel extension.
    func configureVPN(socks5Address: String, completion: @escaping (Error?) -> Void) {
        guard let manager = manager else {
            completion(NSError(domain: "VPNManager", code: -1,
                             userInfo: [NSLocalizedDescriptionKey: "Manager not loaded"]))
            return
        }

        let proto = NETunnelProviderProtocol()
        proto.providerBundleIdentifier = "com.vpndemo.app.VPNDemo.tunnel"
        // serverAddress is required by iOS but it's just a label,
        // the real proxy work happens inside our tunnel extension via tun2socks
        proto.serverAddress = "TUN2SOCKS Local Tunnel"
        // Pass the SOCKS5 address to the extension internally
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
            manager.loadFromPreferences { error in
                completion(error)
            }
        }
    }

    func startVPN(socks5Address: String) throws {
        guard let manager = manager else {
            throw NSError(domain: "VPNManager", code: -1,
                         userInfo: [NSLocalizedDescriptionKey: "Manager not loaded"])
        }

        let session = manager.connection as! NETunnelProviderSession
        // Pass SOCKS5 address as startup option to the extension process
        try session.startVPNTunnel(options: [
            "socks5_address": socks5Address as NSObject
        ])
    }

    func stopVPN() {
        manager?.connection.stopVPNTunnel()
    }

    var status: NEVPNStatus {
        return manager?.connection.status ?? .invalid
    }
}
