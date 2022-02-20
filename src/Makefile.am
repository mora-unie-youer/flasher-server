AUTOMAKE_OPTIONS = subdir-objects

bin_PROGRAMS = flasher-server
flasher_server_CXXFLAGS = $(PTHREAD_CFLAGS)
flasher_server_LDADD = $(PTHREAD_LIBS) -lpqxx
flasher_server_SOURCES = \
	net/tcp.cc \
	main.cc
