"""Tests for the Kaskada query builder."""
import sys

import pyarrow as pa
import pytest
from sparrow_py import math
from sparrow_py.sources import Source


@pytest.fixture(scope="module")
def source1() -> Source:
    """Create a table for testing."""
    schema = pa.schema(
        [
            pa.field("time", pa.int32(), nullable=False),
            pa.field("key", pa.int64(), nullable=False),
            pa.field("x", pa.float64()),
            pa.field("y", pa.int32()),
        ]
    )
    return Source("time", "key", schema)


def test_field_ref(source1) -> None:
    """Test for field references."""
    field_ref_short = source1.x
    assert field_ref_short.data_type == pa.float64()
    assert field_ref_short == source1.x

    field_ref_long = source1["x"]
    assert field_ref_long.data_type == pa.float64()
    assert field_ref_short == field_ref_long


def test_field_ref_no_such_field(source1) -> None:
    """Test error when there is no such field."""
    with pytest.raises(AttributeError) as e:
        # This raises a "NoSuchAttribute" error.
        # We currently catch this in Python and don't do anything to
        # suggest possibly alternatives.
        #
        # TODO: We should either surface the Sparrow error which suggests
        # possible field names, or improve the Python error.
        source1.foo
    assert "Field 'foo' not found in 'time', 'key', 'x', 'y'" == str(e.value)


def test_field_ref_not_a_struct(source1) -> None:
    """Test error when there the base is not a struct."""
    with pytest.raises(TypeError) as e:
        source1.x.x
    assert "Cannot access field 'x' on non-struct type 12" == str(e.value)


def test_expr(source1) -> None:
    """Test creating an expression node."""
    x = source1.x
    assert x + 1 == x + 1


def test_expr_comparison(source1) -> None:
    """Test basic comparisons."""
    assert (source1.x > 1) == (source1.x > 1)

    # Python doesn't have a `__rgt__` (reverse gt) dunder method.
    # Instead, if the LHS doesn't support `gt` with the RHS, it tries
    # rhs `lt` lhs.
    assert (1 < source1.x) == (source1.x > 1)


def test_expr_pipe(source1) -> None:
    """Test using `pipe` to create expressions."""
    assert source1.x.pipe(math.add, 1) == math.add(source1.x, 1)
    assert source1.x.pipe((math.add, "rhs"), 1) == math.add(1, rhs=source1.x)

    assert source1.x.pipe(math.gt, 1) == math.gt(source1.x, 1)
    assert source1.x.pipe((math.gt, "rhs"), 1) == math.gt(1, rhs=source1.x)


def test_expr_arithmetic_types(source1) -> None:
    """Test type inference and type errors of arithmetic expressions."""
    assert math.eq(source1.x, 1).data_type == pa.bool_()
    assert math.add(source1.x, 1).data_type == pa.float64()
    assert (source1.x + source1.y).data_type == pa.float64()

    # TODO: This should raise a TypeError, but currently the Rust
    # code always raises a ValueError, so everything comes out
    # looking the same.
    with pytest.raises(ValueError) as e:
        math.eq(source1.x, 1) + source1.y
    assert "Incompatible argument types" in str(e)
    if sys.version_info >= (3, 11):
        assert "Arg[0]: Expr of type bool" in e.value.__notes__
        assert "Arg[1]: Expr of type int32" in e.value.__notes__
