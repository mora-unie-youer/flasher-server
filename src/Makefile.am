AUTOMAKE_OPTIONS = subdir-objects

bin_PROGRAMS = flasher-server
flasher_server_CXXFLAGS = @PTHREAD_CFLAGS@
flasher_server_LDADD = @PTHREAD_LIBS@ @PQXX_LIBS@
flasher_server_SOURCES = \
	net/addr.cc \
	net/tcp.cc \
	main.cc
