M4 ?=		m4
M4_FLAGS ?=
M4_FLAGS +=	--prefix-builtin

FORCE:
	@:

.PHONY:	FORCE

%/.dirstamp:
	mkdir -p '${@D}'
	@touch '$@'

clean:		clean-common

clean-common:
	rm -f ${CLEANFILES}
