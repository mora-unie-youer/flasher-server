#include "net/tcp.hh"

namespace flasher::net
{
	TcpServer::TcpServer(
		const std::string &name,
		const IpAddr &addr,
		bool reuseAddr,
		bool reusePort
	) : _name(name), _addr(addr)
	{
		// Creating TCP socket
		_socket = socket(
			addr.getFamily(),
			SOCK_STREAM | SOCK_NONBLOCK | SOCK_CLOEXEC,
			IPPROTO_TCP
		);
	}

	TcpServer::~TcpServer()
	{
	}
}
