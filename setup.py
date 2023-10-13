from setuptools import setup, find_packages

setup(
    name="sfp",
    version="0.1.2",
    packages=find_packages(),
    install_requires=[
        'jupyter~=1.0',
        'kedro~=0.18.13',
        'kedro-datasets[pandas.CSVDataSet]~=1.0',
        'kedro-telemetry~=0.2.0'
    ],
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
    description="A tool to make a project structure for a data science simple project",
    long_description=open('README.md').read(),
    long_description_content_type="text/markdown",
    url="https://github.com/Yairama/simple_fast_project",
    license="Apache 2.0",
    license_file="LICENSE.md",

)
