CPP = cpp
CARGO ?= cargo
PATCH = patch
TAR ?= tar
MKDIR_P = mkdir -p
FIND ?= find
INSTALL ?= install
PATCHELF ?= patchelf
DOXYGEN ?= doxygen
INSTALL_DATA = ${INSTALL} -p -m 0644
INSTALL_LIB = ${INSTALL} -p -m 0755

TAR_C_FLAGS = --owner root --group root --mode a+rX,go-w
TAR_X_FLAGS =

_space = $(eval) $(eval)
_comma = ,
