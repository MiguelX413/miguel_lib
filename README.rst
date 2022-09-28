.. badges-start

|CI| |pypi| |pyversion| |implementation| |license| |downloads| |black|

.. |CI| image:: https://github.com/MiguelX413/miguel_lib/actions/workflows/CI.yml/badge.svg
   :alt: GitHub Actions Build Status
   :target: https://github.com/MiguelX413/miguel_lib/actions/workflows/CI.yml

.. |pypi| image:: https://img.shields.io/pypi/v/miguel_lib.svg
   :alt: miguel_lib PyPI Project Page
   :target: https://pypi.org/project/miguel_lib/

.. |license| image:: https://img.shields.io/github/license/MiguelX413/miguel_lib.svg
   :alt: LGPL-2.1 License
   :target: https://github.com/MiguelX413/miguel_lib/blob/master/LICENSE

.. |pyversion| image:: https://img.shields.io/pypi/pyversions/miguel_lib.svg
   :alt: Supported Python Versions

.. |implementation| image:: https://img.shields.io/pypi/implementation/miguel_lib.svg
   :alt: PyPI - Implementation

.. |downloads| image:: https://pepy.tech/badge/miguel_lib/month
   :alt: PyPI Download Count
   :target: https://pepy.tech/project/miguel_lib

.. |black| image:: https://img.shields.io/badge/code%20style-black-000000.svg
   :alt: Code Style Black
   :target: https://github.com/psf/black

.. badges-end

Random crap I like to use

.. code-block:: console

    $ pip install miguel_lib

**miguel_lib**

- ChunksIter: An Iterator which takes an interator and outputs its output in groups the size of a given chunk size

- match_indices(string: str, substring: str) -> List[int]: Returns a list of the UTF-8 indices of disjoint matches, from start to end.

- match_utf16_indices(string: str, substring: str) -> List[int]: Returns a list of the UTF-16 indices of disjoint matches, from start to end.

- match_byte_indices(string: str, substring: str) -> List[int]: Returns a list of the byte indices of disjoint matches, from start to end.

- rmatch_indices(string: str, substring: str) -> List[int]: Returns a list of the UTF-8 indices of disjoint matches, from end to start.

- rmatch_utf16_indices(string: str, substring: str) -> List[int]: Returns a list of the UTF-16 indices of disjoint matches, from end to start.

- rmatch_byte_indices(string: str, substring: str) -> List[int]: Returns a list of the byte indices of disjoint matches, from end to start.

- utf16len(string: str) -> int: A function that returns the UTF-16 length of a string.
