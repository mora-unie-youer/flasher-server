#ifndef __FLASHER__NET__TCP_H__
#define __FLASHER__NET__TCP_H__

#include <netinet/in.h>
#include <string>
#include <sys/socket.h>

namespace flasher::net
{
	// TCP server
	class TcpServer
	{
	private:
		// TCP server name
		std::string _name;
		// TCP server socket
		int _socket;
		// TCP server address
		union
		{
			// TCP IPv4 address
			struct sockaddr_in  ip4;
			// TCP IPv6 address
			struct sockaddr_in6 ip6;
		} _addr;
		// Is TCP address IPv6?
		bool _ipv6;
	public:
		/** Construct new TCP server
		 * @param name name of server
		 * @param addr TCP IPv4/IPv6 address
		 * @param port TCP port
		 * @param ipv6 Is `addr` IPv6?
		 * @param reuseAddr Set SO_REUSEADDR option
		 * @param reusePort Set SO_REUSEPORT option
		 */
		TcpServer(
			const std::string &name,
			const std::string &addr,
			uint16_t port,
			bool ipv6,
			bool reuseAddr = true,
			bool reusePort = true
		);
		~TcpServer();
	};
}

#endif//__FLASHER_SERVER__NET__TCP_H__
