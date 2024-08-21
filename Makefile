TOP_SRCDIR := $(abspath $(dir $(firstword ${MAKEFILE_LIST})))

IS_RELEASE ?=
IS_OFFLINE ?=

srcdir = .

all:

include mk/init.mk

-include ${HOME}/.config/rust/common.mk
-include ${HOME}/.config/rust/etest.mk
-include .local.mk

include mk/tools.mk
include mk/cargo.mk
include mk/grcov.mk
