#ifndef __FLASHER__NET__ADDR_H__
#define __FLASHER__NET__ADDR_H__

#include <netinet/in.h>
#include <string>

namespace flasher::net
{
	// IP address
	class IpAddr
	{
	private:
		// Ip address
		union
		{
			// TCP IPv4 address
			struct sockaddr_in  ip4;
			// TCP IPv6 address
			struct sockaddr_in6 ip6;
		} _addr;
		// Is it IPv6 address?
		bool _ipv6;
	public:
		/**
		 * @brief Construct new IP address
		 *
		 * @param addr ip address
		 * @param port port
		 * @param ipv6 Is `addr` IPv6?
		 */
		IpAddr(const std::string &addr, uint16_t port, bool ipv6);
		~IpAddr();

		/**
		 * @brief Construct a new IPv4 address
		 *
		 * @param addr ipv4 socket address
		 */
		explicit IpAddr(const struct sockaddr_in &addr) : _ipv6(false)
		{
			_addr.ip4 = addr;
		}

		/**
		 * @brief Construct a new IPv6 address
		 *
		 * @param addr ipv6 socket address
		 */
		explicit IpAddr(const struct sockaddr_in6 &addr) : _ipv6(true)
		{
			_addr.ip6 = addr;
		}

		/**
		 * @brief Get family of the endpoint socket
		 *
		 * @return sa_family_t
		 */
		sa_family_t getFamily() const
		{
			// We can use `ip4` because of union
			return _addr.ip4.sin_family;
		}

		/**
		 * @brief Get IP address
		 *
		 * @return std::string
		 */
		std::string getAddr() const;

		/**
		 * @brief Get IP port
		 *
		 * @return uint16_t
		 */
		uint16_t getPort() const;
	};
}

#endif//__FLASHER_SERVER__NET__ADDR_H__
