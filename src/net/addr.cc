#include "net/addr.hh"

#include <arpa/inet.h>
#include <cstring>

namespace flasher::net
{
	IpAddr::IpAddr(const std::string &addr, uint16_t port, bool ipv6)
		: _ipv6(ipv6)
	{
		// IP family
		int family = ipv6 ? AF_INET6 : AF_INET;
		// Filling sockaddr_in[6] depending on `ipv6`
		memset(&_addr, 0, sizeof(_addr));
		// (sockaddr_in[6] are the same, if we look at family and port)
		_addr.ip4.sin_family = family;
		_addr.ip4.sin_port = htons(port);
		// Parsing IP address
		if (inet_pton(
			family,
			addr.c_str(),
			&_addr.ip4.sin_addr
		) <= 0)
			return;
	}

	IpAddr::~IpAddr()
	{
	}

	std::string IpAddr::getAddr() const
	{
		char buf[64];
		// Converting address to string
		inet_ntop(getFamily(), &_addr.ip4.sin_addr, buf, sizeof(buf));
		return buf;
	}

	uint16_t IpAddr::getPort() const
	{
		return htons(_addr.ip4.sin_port);
	}
}
