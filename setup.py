from setuptools import setup, find_packages #, Extension
from setuptools_rust import RustExtension
import sys

if sys.version_info[0] == 3:
    VERSION_FEATURE = "python3-sys"
elif sys.version_info[0] == 2:
    VERSION_FEATURE = "python27-sys"
else:
    raise SystemError("Unknown python major version, this extension expects Python 2 or 3.")

setup(name='mdr',
      version='0.0.1',
      description="python library to detect and extract listing data from HTML page",
      long_description="",
      author="Terry Peng",
      author_email="pengtaoo@gmail.com",
      url='https://github.com/tpeng/mdr',
      license='MIT',
      packages=find_packages(exclude=['tests', 'tests.*']),
      #ext_modules=ext_modules,
      install_requires=['lxml', 'numpy', 'scipy', 'six'],
      rust_extensions=[RustExtension("mdr._treelib", "mdr_treelib/Cargo.toml", features=[VERSION_FEATURE])],
      zip_safe=False
)
