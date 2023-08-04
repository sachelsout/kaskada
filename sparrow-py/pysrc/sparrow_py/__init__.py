"""Kaskada query builder and local executon engine."""
from typing import Dict
from typing import List
from typing import Union

from . import sources
from ._expr import Expr
from ._result import Result
from ._session import init_session
from ._windows import SinceWindow
from ._windows import SlidingWindow
from ._windows import Window


def record(fields: Dict[str, Expr]) -> Expr:
    """Create a record from the given keyword arguments."""
    import itertools

    args: List[Union[str, "Expr"]] = list(itertools.chain(*fields.items()))
    return Expr.call("record", *args)


__all__ = [
    "Expr",
    "init_session",
    "record",
    "Window",
    "SinceWindow",
    "SlidingWindow",
    "sources",
    "Result",
]
