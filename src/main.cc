#include <iostream>

#include "net/tcp.hh"

int main(int argc, char **argv)
{
	using namespace flasher::net;
	std::cout << "Hello, world!" << std::endl;
	IpAddr addr("313:c0de:ceca:57f4::2", 5555, true);
	std::cout << addr.getAddr() << std::endl;
	std::cout << addr.getPort() << std::endl;
	std::cout << addr.getFamily() << std::endl;
	TcpServer server = TcpServer("SomeTestServer", addr);
	std::cout << &server << std::endl;
	return 0;
}
