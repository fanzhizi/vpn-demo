#ifndef TUNNEL_CORE_H
#define TUNNEL_CORE_H

#include <stdbool.h>
#include <stddef.h>

/// Start the tunnel with a SOCKS5 proxy address string (e.g. "1.2.3.4:1080").
bool tunnel_start(const char *socks5_addr);

/// Stop the tunnel.
void tunnel_stop(void);

/// Feed a raw IP packet into the tunnel (from TUN device).
bool tunnel_feed_packet(const unsigned char *data, size_t len);

/// Try to read an outbound packet. Returns bytes written, or 0 if none available.
size_t tunnel_read_packet(unsigned char *buf, size_t buf_len);

/// Check if tunnel is running.
bool tunnel_is_running(void);

#endif /* TUNNEL_CORE_H */
