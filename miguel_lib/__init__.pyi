from typing import Protocol

from .miguel_lib import *

__doc__ = miguel_lib.__doc__

class SupportsSpan(Protocol):
    def to_span(self) -> miguel_lib.Span:
        pass

class SupportsInterval(Protocol):
    def to_interval(self) -> miguel_lib.Interval:
        pass

__all__ = ["SupportsSpan", "SupportsInterval"]
if hasattr(miguel_lib, "__all__"):
    __all__ += miguel_lib.__all__
