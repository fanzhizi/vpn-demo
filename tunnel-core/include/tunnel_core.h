#ifndef TUNNEL_CORE_H
#define TUNNEL_CORE_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

bool tunnel_start(const char *socks5_addr);
void tunnel_stop(void);
bool tunnel_feed_packet(const unsigned char *data, size_t len);
size_t tunnel_read_packet(unsigned char *buf, size_t buf_len);
bool tunnel_is_running(void);

/// Test SOCKS5 connectivity directly. Returns 1 on success, 0 on failure.
/// Result message written to out_buf (null-terminated).
int32_t tunnel_test_socks5(const char *socks5_addr, unsigned char *out_buf, size_t out_len);

#endif /* TUNNEL_CORE_H */
