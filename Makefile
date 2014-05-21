MY_MAKEFILE := $(abspath $(lastword $(MAKEFILE_LIST)))
MY_DIR := $(dir $(MY_MAKEFILE))

ROOT := $(MY_DIR)
SRCDIR := $(ROOT)src/
BINDIR := $(ROOT)bin/
OBJDIR := $(ROOT)obj/

DEPINFODIR := $(OBJDIR)dep-info/

RUSTC := rustc

RUSTFLAGS :=

-include $(DEPINFODIR)rave.d

all: $(BINDIR)rave

$(OBJDIR):
	@mkdir $(OBJDIR)

$(DEPINFODIR): $(OBJDIR)
	@mkdir $(DEPINFODIR)

$(BINDIR):
	@mkdir $(BINDIR)

$(BINDIR)rave: $(SRCDIR)rave/rave.rs | $(BINDIR) $(DEPINFODIR)
	$(RUSTC) -L $(BINDIR) --out-dir $(BINDIR) --dep-info $(DEPINFODIR)rave.d $<

clean:
	@rm -Rf $(BINDIR)
	@rm -Rf $(OBJDIR)