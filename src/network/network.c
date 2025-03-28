#include <stdint.h>
#include <stdio.h>

#include <netinet/ip.h>
#include <unistd.h>

void forward_packet_ipv4(uint8_t *buf, int len) {
    int sock_ipv4 = socket(AF_INET, SOCK_RAW, IPPROTO_RAW);
    if (sock_ipv4 < 0) {
        perror("Failed to create socket");
        return;
    }

    int opt = 1;
    setsockopt(sock_ipv4, IPPROTO_IP, IP_HDRINCL, &opt, sizeof(opt));

    struct iphdr *header = (struct iphdr *)buf;
    struct sockaddr_in destination;

    destination.sin_family = AF_INET;
    destination.sin_addr.s_addr = header->daddr;

    if (sendto(sock_ipv4, buf, len, 0, (struct sockaddr *)&destination, sizeof(destination)) < 0) {
        perror("Failed to send packet to IPv4 destination");
    }
}