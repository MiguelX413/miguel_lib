"""Random crap I like to use"""

from typing import Generic, Iterable, List, TypeVar

_T = TypeVar("_T")

__version__: str

class ChunksIter(Generic[_T]):
    """An Iterator which takes an interator and outputs its output in groups the size of a given chunk size"""

    def __new__(cls, iter: Iterable[_T], chunk_size: int) -> ChunksIter[_T]: ...
    def __iter__(self) -> ChunksIter[_T]: ...
    def __next__(self) -> List[_T]: ...

def match_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the UTF-8 indices of disjoint matches, from start to end."""

def match_utf16_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the UTF-16 indices of disjoint matches, from start to end."""

def match_byte_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the byte indices of disjoint matches, from start to end."""

def rmatch_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the UTF-8 indices of disjoint matches, from end to start."""

def rmatch_utf16_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the UTF-16 indices of disjoint matches, from end to start."""

def rmatch_byte_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the byte indices of disjoint matches, from end to start."""

def utf16len(string: str) -> int:
    """A function that returns the UTF-16 length of a string."""
