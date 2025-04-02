#include <stdint.h>

void forward_packet_ipv4(uint8_t *buf, uint32_t len);
int32_t ip_socket();
int32_t socket_read(int32_t sock, uint8_t *buf, int32_t len);