MY_MAKEFILE := $(abspath $(lastword $(MAKEFILE_LIST)))
MY_DIR := $(dir $(MY_MAKEFILE))

ROOT := $(MY_DIR)
SRCDIR := $(ROOT)src/
BINDIR := $(ROOT)bin/
OBJDIR := $(ROOT)obj/

DEPINFODIR := $(OBJDIR)dep-info/

RUSTC := rustc

RUSTFLAGS :=

CRATE := $(SRCDIR)rave/bin.rs

-include $(DEPINFODIR)rave.d

all: $(BINDIR)rave

$(OBJDIR):
	@mkdir $(OBJDIR)

$(DEPINFODIR): $(OBJDIR)
	@mkdir $(DEPINFODIR)

$(BINDIR):
	@mkdir $(BINDIR)

$(BINDIR)rave: $(CRATE) | $(BINDIR) $(DEPINFODIR)
	$(RUSTC) -L $(BINDIR) -o $(BINDIR)rave --dep-info $(DEPINFODIR)rave.d $<

clean:
	@rm -Rf $(BINDIR)
	@rm -Rf $(OBJDIR)

$(BINDIR)rave-test: $(CRATE) | $(BINDIR) $(DEPINFODIR)
	$(RUSTC) -L $(BINDIR) --test -o $(BINDIR)rave-test --dep-info $(DEPINFODIR)rave.d $<

test: $(BINDIR)rave-test
	$(BINDIR)/rave-test