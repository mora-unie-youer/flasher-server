#include <iostream>

#include "net/tcp.hh"

int main(int argc, char **argv)
{
	using namespace flasher::net;
	std::cout << "Hello, world!" << std::endl;
	TcpServer server = TcpServer("SomeTestServer", "127.0.0.1", 5555, false);
	return 0;
}
