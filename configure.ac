#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.69])
AC_INIT([flasher-server], [0.0.0], [mora_unie_youer@riseup.net microchipq@gmail.com])
AC_LANG([C++])
AC_CONFIG_AUX_DIR([aux-dist])
AC_CONFIG_MACRO_DIR([m4])
AM_INIT_AUTOMAKE

# Checks for programs.
AC_PROG_MAKE_SET
AC_PROG_CXX
AC_PROG_CC

# Checks for libraries.
AX_PTHREAD([], [AC_MSG_ERROR(`libpthread` is required)])

# Checks for header files.
AC_CHECK_HEADERS([libpq-fe.h pqxx/pqxx], [
	PQXX_LIBS="-lpq -lpqxx"
	AC_SUBST([PQXX_LIBS])
], [AC_MSG_ERROR(`libpq` and `libpqxx` are required)])

# Checks for typedefs, structures, and compiler characteristics.

# Checks for library functions.

AC_CONFIG_FILES([Makefile src/Makefile])
AC_OUTPUT
