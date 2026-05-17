import NetworkExtension
import os.log

class PacketTunnelProvider: NEPacketTunnelProvider {

    private let log = OSLog(subsystem: "com.vpndemo.app.tunnel", category: "tunnel")
    private var readTimer: DispatchSourceTimer?
    private var packetCount: UInt64 = 0
    private var outPacketCount: UInt64 = 0

    override func startTunnel(options: [String: NSObject]?, completionHandler: @escaping (Error?) -> Void) {
        os_log("=== PacketTunnelProvider startTunnel ===", log: log, type: .info)

        let socks5Address: String
        if let addr = options?["socks5_address"] as? String {
            socks5Address = addr
            os_log("SOCKS5 from options: %{public}@", log: log, type: .info, socks5Address)
        } else if let proto = protocolConfiguration as? NETunnelProviderProtocol,
                  let addr = proto.providerConfiguration?["socks5_address"] as? String {
            socks5Address = addr
            os_log("SOCKS5 from config: %{public}@", log: log, type: .info, socks5Address)
        } else {
            os_log("ERROR: No SOCKS5 address found!", log: log, type: .error)
            completionHandler(NSError(domain: "PacketTunnel", code: -1,
                                     userInfo: [NSLocalizedDescriptionKey: "No SOCKS5 address"]))
            return
        }

        os_log("Calling tunnel_start...", log: log, type: .info)
        let started = socks5Address.withCString { ptr in
            tunnel_start(ptr)
        }

        if !started {
            os_log("ERROR: tunnel_start returned false!", log: log, type: .error)
            completionHandler(NSError(domain: "PacketTunnel", code: -2,
                                     userInfo: [NSLocalizedDescriptionKey: "Failed to start Rust tunnel"]))
            return
        }
        os_log("tunnel_start succeeded", log: log, type: .info)

        let settings = NEPacketTunnelNetworkSettings(tunnelRemoteAddress: "10.0.0.1")

        let ipv4 = NEIPv4Settings(addresses: ["10.0.0.2"], subnetMasks: ["255.255.255.0"])
        ipv4.includedRoutes = [NEIPv4Route.default()]
        // No excludedRoutes needed for the proxy server IP:
        // iOS NE Extension process sockets automatically bypass the TUN
        // (system-level isolation, unlike Android which needs protect(fd))
        settings.ipv4Settings = ipv4

        settings.dnsSettings = NEDNSSettings(servers: ["8.8.8.8", "8.8.4.4"])
        settings.mtu = 1500

        os_log("Applying tunnel network settings...", log: log, type: .info)
        setTunnelNetworkSettings(settings) { [weak self] error in
            guard let self = self else { return }
            if let error = error {
                os_log("ERROR setting tunnel: %{public}@", log: self.log, type: .error, error.localizedDescription)
                completionHandler(error)
                return
            }

            os_log("Tunnel settings applied OK, starting packet flow", log: self.log, type: .info)
            self.startReadingPackets()
            self.startWritingPackets()
            completionHandler(nil)
        }
    }

    override func stopTunnel(with reason: NEProviderStopReason, completionHandler: @escaping () -> Void) {
        os_log("stopTunnel reason: %d", log: log, type: .info, reason.rawValue)
        readTimer?.cancel()
        readTimer = nil
        tunnel_stop()
        completionHandler()
    }

    private func startReadingPackets() {
        packetFlow.readPackets { [weak self] packets, protocols in
            guard let self = self else { return }

            for (i, packet) in packets.enumerated() {
                self.packetCount += 1
                if self.packetCount <= 5 || self.packetCount % 100 == 0 {
                    os_log("IN packet #%llu: %lu bytes, proto=%{public}@",
                           log: self.log, type: .debug,
                           self.packetCount,
                           packet.count,
                           protocols[i].stringValue)
                }

                packet.withUnsafeBytes { ptr in
                    guard let baseAddr = ptr.baseAddress else { return }
                    let ok = tunnel_feed_packet(
                        baseAddr.assumingMemoryBound(to: UInt8.self),
                        ptr.count
                    )
                    if !ok && self.packetCount <= 5 {
                        os_log("tunnel_feed_packet FAILED for packet #%llu", log: self.log, type: .error, self.packetCount)
                    }
                }
            }

            // Continue reading
            self.startReadingPackets()
        }
    }

    private func startWritingPackets() {
        let timer = DispatchSource.makeTimerSource(queue: DispatchQueue.global(qos: .userInteractive))
        timer.schedule(deadline: .now(), repeating: .milliseconds(1))

        timer.setEventHandler { [weak self] in
            guard let self = self else { return }

            var buf = [UInt8](repeating: 0, count: 65535)
            let len = tunnel_read_packet(&buf, buf.count)

            if len > 0 {
                self.outPacketCount += 1
                if self.outPacketCount <= 5 || self.outPacketCount % 100 == 0 {
                    os_log("OUT packet #%llu: %lu bytes", log: self.log, type: .debug, self.outPacketCount, len)
                }
                let packetData = Data(bytes: buf, count: len)
                self.packetFlow.writePackets([packetData], withProtocols: [NSNumber(value: AF_INET)])
            }
        }

        timer.resume()
        readTimer = timer
    }
}
