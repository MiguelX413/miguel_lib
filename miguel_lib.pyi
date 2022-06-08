from typing import Optional, Sequence, Tuple

def merge_intervals(intervals: Sequence[Tuple[int, int]]) -> Sequence[Tuple[int, int]]:
    """A function that merges overlapping intervals in a sequence."""

def utf16len(string: str) -> int:
    """A function that returns the UTF-16 length of a string."""

class Interval:
    """A class used to represent intervals."""

    def __init__(
        self, interval_list: Optional[Sequence[Tuple[int, int]]] = None
    ) -> None: ...
    def union_update(self, other: Interval) -> None: ...
    def __contains__(self, item: int) -> bool: ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
