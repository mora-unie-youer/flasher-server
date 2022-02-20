#include <iostream>
#include <thread>

#include "net/tcp.hh"

int main(int argc, char **argv)
{
	using namespace flasher::net;
	std::cout << "Hello, world!" << std::endl;
	std::thread tcp = std::thread([] {
		TcpServer server = TcpServer("SomeTestServer", "127.0.0.1", 5555, false);
		std::cout << &server << std::endl;
	});
	std::cout << "Hello" << std::endl;
	tcp.join();
	return 0;
}
