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
		// IP family
		int family = ipv6 ? AF_INET6 : AF_INET;
		// Creating TCP socket
		_socket = socket(
			family,
			SOCK_STREAM | SOCK_NONBLOCK | SOCK_CLOEXEC,
			IPPROTO_TCP
		);
		// Filling sockaddr_in[6] depending on `ipv6`
		memset(&_addr, 0, sizeof(_addr));
		// (sockaddr_in[6] are the same, if we look at family and port)
		_addr.ip4.sin_family = family;
		_addr.ip4.sin_port = htons(port);
		// Parsing IP address
		if (inet_pton(
			family,
			addr.c_str(),
			ipv6 ? (void*) &_addr.ip6.sin6_addr : (void*) &_addr.ip4.sin_addr
		) <= 0)
			return;
	}

	TcpServer::~TcpServer()
	{
	}
}
