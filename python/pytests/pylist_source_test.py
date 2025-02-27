import kaskada as kd


def test_read_pylist(golden) -> None:
    source = kd.sources.PyList(
        [{"time": "1996-12-19T16:39:57", "user": "A", "m": 5, "n": 10}],
        time_column_name="time",
        key_column_name="user",
    )
    golden.jsonl(source)

    source.add_rows(
        [
            {"time": "1996-12-19T16:40:57", "user": "A", "m": 8, "n": 10},
            {"time": "1996-12-19T16:41:57", "user": "B", "m": 5},
        ]
    )

    golden.jsonl(source)
    source.add_rows({"time": "1996-12-19T16:42:57", "user": "A", "m": 8, "n": 10})
    golden.jsonl(source)


def test_read_pylist_lists(golden) -> None:
    source = kd.sources.PyList(
        [{"time": "1996-12-19T16:39:57", "user": "A", "m": [5, 10], "n": 10}],
        time_column_name="time",
        key_column_name="user",
    )
    golden.jsonl(source)

    source.add_rows(
        [
            {"time": "1996-12-19T16:40:57", "user": "A", "m": [], "n": 10},
            {"time": "1996-12-19T16:41:57", "user": "A", "n": 10},
        ]
    )
    golden.jsonl(source)
