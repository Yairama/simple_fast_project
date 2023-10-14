# Detect OS
ifeq ($(wildcard /bin/uname),)
    # Windows
    RM = del /Q /F
    CP = xcopy /Y
    SLASH = \\
    EXE = .exe
    INSTALL_WHEELS = for %%f in (dist$(SLASH)sfp-*.whl) do python -m pip install %%f --force-reinstall
else
    # Linux
    RM = rm -rf
    CP = cp
    SLASH = /
    EXE =
    INSTALL_WHEELS = for f in dist$(SLASH)sfp-*.whl; do pip install $$f --force-reinstall; done
endif

# Paths
RUST_TARGET = target$(SLASH)release$(SLASH)sfp$(EXE)
PYTHON_DIR = simple_fast_project$(SLASH)

all: build move_to_python build_python_package install_python_package

python_test: clean build move_to_python build_python_package install_python_package

build:
	@echo "Building the Rust project..."
	cargo build --release

move_to_python:
	@echo "Moving the executable file to the Python folder..."
	$(CP) $(RUST_TARGET) $(PYTHON_DIR)

build_python_package:
	@echo "Building the Python package..."
	python setup.py sdist bdist_wheel

install_python_package:
	@echo "Installing the Python packages..."
	$(INSTALL_WHEELS)

upload_pypi:
	@echo "Uploading the distribution to PyPI..."
	pip install twine
	twine upload --config-file .pypirc dist$(SLASH)*

clean:
	@echo "Cleaning up generated files..."
	$(RM) dist$(SLASH)* build$(SLASH)* sfp.egg-info$(SLASH)*

.PHONY: all build move_to_python build_python_package install_python_package upload_pypi clean
