import NetworkExtension
import os.log

/// NEPacketTunnelProvider implementation that bridges to the Rust tunnel-core library.
/// This runs in a separate process from the main app.
class PacketTunnelProvider: NEPacketTunnelProvider {

    private let log = OSLog(subsystem: "com.vpndemo.app.tunnel", category: "tunnel")
    private var readTimer: DispatchSourceTimer?

    override func startTunnel(options: [String: NSObject]?, completionHandler: @escaping (Error?) -> Void) {
        os_log("Starting tunnel", log: log, type: .info)

        // Get SOCKS5 address from options or protocol config
        let socks5Address: String
        if let addr = options?["socks5_address"] as? String {
            socks5Address = addr
        } else if let proto = protocolConfiguration as? NETunnelProviderProtocol,
                  let addr = proto.providerConfiguration?["socks5_address"] as? String {
            socks5Address = addr
        } else {
            completionHandler(NSError(domain: "PacketTunnel", code: -1,
                                     userInfo: [NSLocalizedDescriptionKey: "No SOCKS5 address"]))
            return
        }

        os_log("SOCKS5 proxy: %{public}@", log: log, type: .info, socks5Address)

        // Start the Rust tunnel core
        let started = socks5Address.withCString { ptr in
            tunnel_start(ptr)
        }

        guard started else {
            completionHandler(NSError(domain: "PacketTunnel", code: -2,
                                     userInfo: [NSLocalizedDescriptionKey: "Failed to start Rust tunnel"]))
            return
        }

        // Configure TUN interface settings
        let settings = NEPacketTunnelNetworkSettings(tunnelRemoteAddress: "10.0.0.1")

        // IPv4 settings - route all traffic through tunnel
        let ipv4 = NEIPv4Settings(addresses: ["10.0.0.2"], subnetMasks: ["255.255.255.0"])
        ipv4.includedRoutes = [NEIPv4Route.default()]
        // Exclude the SOCKS5 server itself from the tunnel
        if let components = parseAddress(socks5Address) {
            let excludeRoute = NEIPv4Route(destinationAddress: components.host, subnetMask: "255.255.255.255")
            ipv4.excludedRoutes = [excludeRoute]
        }
        settings.ipv4Settings = ipv4

        // DNS settings
        let dns = NEDNSSettings(servers: ["8.8.8.8", "8.8.4.4"])
        settings.dnsSettings = dns

        settings.mtu = 1500

        // Apply network settings
        setTunnelNetworkSettings(settings) { [weak self] error in
            if let error = error {
                os_log("Failed to set tunnel settings: %{public}@",
                       log: self?.log ?? .default, type: .error, error.localizedDescription)
                completionHandler(error)
                return
            }

            os_log("Tunnel settings applied, starting packet flow", log: self?.log ?? .default, type: .info)

            // Start reading packets from TUN device
            self?.startReadingPackets()

            // Start writing packets back to TUN device
            self?.startWritingPackets()

            completionHandler(nil)
        }
    }

    override func stopTunnel(with reason: NEProviderStopReason, completionHandler: @escaping () -> Void) {
        os_log("Stopping tunnel, reason: %d", log: log, type: .info, reason.rawValue)

        readTimer?.cancel()
        readTimer = nil

        tunnel_stop()

        completionHandler()
    }

    // MARK: - Packet Flow

    /// Read packets from TUN device and feed them to Rust tunnel core.
    private func startReadingPackets() {
        packetFlow.readPackets { [weak self] packets, protocols in
            guard let self = self else { return }

            for packet in packets {
                packet.withUnsafeBytes { ptr in
                    guard let baseAddr = ptr.baseAddress else { return }
                    let _ = tunnel_feed_packet(
                        baseAddr.assumingMemoryBound(to: UInt8.self),
                        ptr.count
                    )
                }
            }

            // Continue reading
            self.startReadingPackets()
        }
    }

    /// Poll Rust tunnel core for outbound packets and write them to TUN device.
    private func startWritingPackets() {
        let timer = DispatchSource.makeTimerSource(queue: DispatchQueue.global(qos: .userInteractive))
        timer.schedule(deadline: .now(), repeating: .milliseconds(1))

        timer.setEventHandler { [weak self] in
            guard let self = self else { return }

            var buf = [UInt8](repeating: 0, count: 65535)
            let len = tunnel_read_packet(&buf, buf.count)

            if len > 0 {
                let packetData = Data(bytes: buf, count: len)
                // Determine protocol (IPv4 = AF_INET = 2)
                let proto = NSNumber(value: AF_INET)
                self.packetFlow.writePackets([packetData], withProtocols: [proto])
            }
        }

        timer.resume()
        readTimer = timer
    }

    // MARK: - Helpers

    private func parseAddress(_ addr: String) -> (host: String, port: String)? {
        let parts = addr.split(separator: ":")
        guard parts.count == 2 else { return nil }
        return (host: String(parts[0]), port: String(parts[1]))
    }
}
