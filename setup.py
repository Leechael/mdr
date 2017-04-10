from setuptools import setup, find_packages #, Extension
from setuptools_rust import RustExtension
import sys


setup(name='mdr',
      version='0.0.1',
      description="python library to detect and extract listing data from HTML page",
      long_description="",
      author="Terry Peng",
      author_email="pengtaoo@gmail.com",
      url='https://github.com/tpeng/mdr',
      license='MIT',
      packages=find_packages(exclude=['tests', 'tests.*']),
      install_requires=['lxml', 'numpy', 'scipy', 'six'],
      rust_extensions=[RustExtension("mdr._treelib", "mdr_treelib/Cargo.toml")],
      zip_safe=False
)
