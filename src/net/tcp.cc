#include "tcp.hh"

#include <arpa/inet.h>
#include <cstring>

namespace flasher::net {
	TcpServer::TcpServer(
		const std::string &name,
		const std::string &addr,
		uint16_t port,
		bool ipv6,
		bool reuseAddr,
		bool reusePort
	) : _name(name), _ipv6(ipv6)
	{
		// Filling sockaddr_in[6] depending on `ipv6`
		if (ipv6)
		{
			// Clearing sockaddr_in6
			memset(&_addr.ip6, 0, sizeof(_addr.ip6));
			// Setting family
			_addr.ip6.sin6_family = AF_INET6;
			// Parsing IPv6 address from string
			if (inet_pton(AF_INET6, addr.c_str(), &_addr.ip6.sin6_addr) <= 0)
				return;
			// Setting port
			_addr.ip6.sin6_port = htons(port);
		}
		else
		{
			// Clearing sockaddr_in
			memset(&_addr.ip4, 0, sizeof(_addr.ip4));
			// Setting family
			_addr.ip4.sin_family = AF_INET;
			// Parsing IPv4 address from string
			if (inet_pton(AF_INET, addr.c_str(), &_addr.ip4.sin_addr) <= 0)
				return;
			// Setting port
			_addr.ip4.sin_port = htons(port);
		}
	}

	TcpServer::~TcpServer()
	{
	}
}
