from setuptools import setup, find_packages #, Extension
from rust_ext import build_rust_cmdclass, install_lib_including_rust
#import numpy

#ext_modules = [Extension('mdr._tree',
#    sources=['mdr/_tree.c'],
#    include_dirs = [numpy.get_include()],
#)]

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
      install_requires=['lxml'],
      cmdclass={
          'build_rust': build_rust_cmdclass('treefuncs/Cargo.toml'),
          '': install_lib_including_rust
      },
      zip_safe=False
)
