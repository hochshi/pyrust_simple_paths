#!/usr/bin/env python

from distutils.command.build import build as build_orig

import pkg_resources
# from distutils.extension import Extension
from setuptools import Extension, find_packages, setup
from setuptools_rust import RustExtension

# def finalize_distribution_options(dist):
#    print('###################################')
#    print(vars(dist))
#    print('###################################')
#    import pdb; pdb.set_trace()

# class build(build_orig):
#    def finalize_options(self):
#        super().finalize_options()
#        try:
#            __builtins__.__NUMPY_SETUP__ = False
#        except AttributeError:
#            pass
#        ext = next(m for m in self.distribution.ext_modules if m == ext_modules[0])
#        ext.include_dirs.append(
#            pkg_resources.resource_filename("numpy", "core/include")
#        )


#
# directive_defaults = Options.get_directive_defaults()
# directive_defaults['profile'] = True
# directive_defaults['linetrace'] = True
# directive_defaults['binding'] = True

# USE_CYTHON = "auto"
#
# if USE_CYTHON:
#    try:
#        from Cython.Build import cythonize
#        from Cython.Compiler import Options
#
#        Options.docstrings = True
#    except ImportError:
#        if USE_CYTHON == "auto":
#            USE_CYTHON = False
#        else:
#            raise
#
# file_ext = ".pyx" if USE_CYTHON else ".cpp"
#
# ext_modules = [
#    Extension(
#        "dawdlib.dijkstra.colorful_algorithm",
#        ["dawdlib/dijkstra/colorful_algorithm" + file_ext],
#        include_dirs=[],
#        language="c++",
#        extra_compile_args=["-Ofast", "-std=c++11"]
#        # define_macros=[('CYTHON_TRACE', '1'), ('CYTHON_TRACE_NOGIL', '1')]
#    )
# ]
#
# if USE_CYTHON:
#    ext_modules = cythonize(ext_modules, gdb_debug=False)


setup(
    name="pyrustsimplepaths",
    version="0.0.1a0",
    packages=find_packages(),
    zip_safe=False,
    # ext_modules=ext_modules,
    # cmdclass={"build": build},
    install_requires=[],
    rust_extensions=[
        RustExtension("smp.r_simple_paths", "Cargo.toml", debug=False)
    ],
    python_requires=">=3.7",
    setup_requires=["setuptools", "setuptools-rust"],
    include_package_data=True
)
