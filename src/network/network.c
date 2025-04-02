#include <stdint.h>
#include <stdio.h>

#include <netinet/ip.h>
#include <netinet/ether.h>
#include <unistd.h>

int32_t ip_socket() {
    int fd = socket(AF_PACKET, SOCK_RAW, htons(ETH_P_IP));
    printf("Socket created\n");
    if (fd < 0) {
        perror("Failed to create socket");
        return fd;
    }

    // if (setsockopt(fd, SOL_SOCKET, SO_ATTACH_FILTER, &filter, sizeof(struct sock_fprog)) < 0) {
    //     perror("Failed to set socket filter");
    // }

    return fd;
}

int32_t socket_read(int32_t sock, uint8_t *buf, int32_t len) {
    printf("read from c\n");
    int bytes = recv(sock, buf, len, 0);
    printf("After read from c\n");
    return bytes;
}

void forward_packet_ipv4(uint8_t *buf, int len) {
    int sock_ipv4 = socket(AF_INET, SOCK_RAW, IPPROTO_IP);
    if (sock_ipv4 < 0) {
        perror("Failed to create socket");
        return;
    }

    int opt = 1;
    if (setsockopt(sock_ipv4, IPPROTO_IP, IP_HDRINCL, &opt, sizeof(opt)) < 0) {
        perror("Failed to set socket option");
    }

    struct iphdr *header = (struct iphdr *)buf;
    struct sockaddr_in destination;

    destination.sin_family = AF_INET;
    destination.sin_addr.s_addr = header->daddr;

    if (sendto(sock_ipv4, buf, len, 0, (struct sockaddr *)&destination, sizeof(destination)) < 0) {
        perror("Failed to send packet to IPv4 destination");
    }
}