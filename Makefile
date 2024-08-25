PYTHON = python3

VENV      = venv
ACTIVATE := . $(VENV)/bin/activate

# Write a marker .install file to indicate that the dependencies have been
# installed.
INST := $(VENV)/.install
$(INST): requirements
	$(PYTHON) -m venv $(VENV)
	$(ACTIVATE); pip install --upgrade pip
	$(ACTIVATE); pip install -r requirements/dev.txt
	touch $@

.PHONY: install
install: $(INST)

.PHONY: clean
clean:
	rm -rf __pycache__ venv

.PHONY: fmt
fmt: install
	$(ACTIVATE); black ./

.PHONY: lint
lint: install
	$(ACTIVATE); pylint src/

.PHONY: types
types: install
	$(ACTIVATE); mypy src/ --strict

.PHONY: check
check: fmt lint types
