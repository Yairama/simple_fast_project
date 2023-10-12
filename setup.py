from setuptools import setup, find_packages

setup(
    name="sfp",
    version="0.1",
    packages=find_packages(),
    install_requires=[],
    entry_points={
        'console_scripts': [
            'sfp = simple_fast_project.main:run',
        ],
    },
    include_package_data=True,
    package_data={
        '': ['*.exe'],
    },
    author="Yair Camborda",
    author_email="yairoriginal@gmail.com",
    description="Una herramienta para ejecutar sfp.exe",
    long_description=open('README.md').read(),
    long_description_content_type="text/markdown",
    url="https://https://github.com/Yairama/simple_fast_project",
    license="Apache 2.0",
    license_file="LICENSE.md"
)
