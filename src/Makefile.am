AUTOMAKE_OPTIONS = subdir-objects

bin_PROGRAMS = flasher-server
flasher_server_LDADD = -lpthread -lpqxx
flasher_server_SOURCES = \
	net/tcp.cc \
	main.cc
