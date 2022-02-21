#ifndef __FLASHER__NET__TCP_H__
#define __FLASHER__NET__TCP_H__

#include <string>
#include <sys/socket.h>

#include "net/addr.hh"

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
		IpAddr _addr;
	public:
		/**
		 * @brief Construct new TCP server
		 *
		 * @param name name of server
		 * @param addr TCP server address
		 * @param reuseAddr Set SO_REUSEADDR option
		 * @param reusePort Set SO_REUSEPORT option
		 */
		TcpServer(
			const std::string &name,
			const IpAddr &addr,
			bool reuseAddr = true,
			bool reusePort = true
		);
		~TcpServer();
	};
}

#endif//__FLASHER_SERVER__NET__TCP_H__
