# SPDX-License-Identifier: (Apache-2.0 OR MIT)

import pytest

import orjson


class TestLoadsMultiple:
    def test_basic_multiple_documents(self):
        """
        loads_multiple() parses multiple JSON documents
        """
        data = '{"foo":"bar"}\n{"baz":"quux"}\n{"spam":"eggs"}'
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"foo": "bar"}, {"baz": "quux"}, {"spam": "eggs"}]

    def test_lazy_evaluation(self):
        """
        loads_multiple() returns a lazy iterator
        """
        data = '{"foo":"bar"}\n{"baz":"quux"}\n{"spam":"eggs"}'
        iterator = orjson.loads_multiple(data)

        # Get first document
        first = next(iterator)
        assert first == {"foo": "bar"}

        # Get second document
        second = next(iterator)
        assert second == {"baz": "quux"}

        # Can stop early without parsing the rest
        # Third document is never parsed

    def test_whitespace_handling(self):
        """
        loads_multiple() handles whitespace between documents
        """
        data = '{"a":1}  \n\n  {"b":2}\t\t{"c":3}'
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"a": 1}, {"b": 2}, {"c": 3}]

    def test_different_types(self):
        """
        loads_multiple() parses different JSON types
        """
        data = '{"obj": true}\n[1,2,3]\n"string"\n123\nnull\ntrue\nfalse'
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"obj": True}, [1, 2, 3], "string", 123, None, True, False]

    def test_only_whitespace(self):
        """
        loads_multiple() handles only whitespace
        """
        data = "   \n\n\t  "
        docs = list(orjson.loads_multiple(data))
        assert docs == []

    def test_empty_string(self):
        """
        loads_multiple() handles empty string
        """
        data = ""
        docs = list(orjson.loads_multiple(data))
        assert docs == []

    def test_single_document(self):
        """
        loads_multiple() handles single document
        """
        data = '{"single": "document"}'
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"single": "document"}]

    def test_bytes_input(self):
        """
        loads_multiple() works with bytes input
        """
        data = b'{"foo":"bar"}\n{"baz":"quux"}'
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"foo": "bar"}, {"baz": "quux"}]

    def test_error_handling(self):
        """
        loads_multiple() raises JSONDecodeError for invalid JSON
        """
        data = '{"valid": true}\n{invalid json}\n{"more": "data"}'
        iterator = orjson.loads_multiple(data)

        # First document should parse fine
        first = next(iterator)
        assert first == {"valid": True}

        # Second document should raise an error
        with pytest.raises(orjson.JSONDecodeError):
            next(iterator)

    def test_ndjson_use_case(self):
        """
        loads_multiple() handles typical NDJSON/JSONL use case
        """
        # Simulate NDJSON log data
        data = """{"timestamp": "2024-01-01", "level": "INFO", "message": "Server started"}
{"timestamp": "2024-01-01", "level": "ERROR", "message": "Connection failed"}
{"timestamp": "2024-01-01", "level": "INFO", "message": "Retrying..."}"""

        errors = [
            doc for doc in orjson.loads_multiple(data) if doc.get("level") == "ERROR"
        ]
        assert errors == [
            {
                "timestamp": "2024-01-01",
                "level": "ERROR",
                "message": "Connection failed",
            }
        ]

    def test_no_newlines(self):
        """
        loads_multiple() works without newline separators
        """
        data = '{"a":1}{"b":2}{"c":3}'
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"a": 1}, {"b": 2}, {"c": 3}]

    def test_nested_objects(self):
        """
        loads_multiple() handles nested objects
        """
        data = '{"a":{"b":{"c":1}}}\n{"x":[1,2,{"y":3}]}'
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"a": {"b": {"c": 1}}}, {"x": [1, 2, {"y": 3}]}]

    def test_unicode_strings(self):
        """
        loads_multiple() handles unicode strings
        """
        data = '{"emoji":"😀"}\n{"text":"hello 世界"}'
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"emoji": "😀"}, {"text": "hello 世界"}]

    def test_large_numbers(self):
        """
        loads_multiple() handles large numbers
        """
        data = '{"big":9007199254740991}\n{"small":-9007199254740991}'
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"big": 9007199254740991}, {"small": -9007199254740991}]

    def test_trailing_content_after_documents(self):
        """
        loads_multiple() handles trailing whitespace after all documents
        """
        data = '{"a":1}\n{"b":2}\n\n  \t  '
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"a": 1}, {"b": 2}]

    def test_mixed_whitespace(self):
        """
        loads_multiple() handles mixed whitespace (spaces, tabs, newlines)
        """
        data = '  \t{"a":1}  \n\t  \n{"b":2}\r\n{"c":3}  '
        docs = list(orjson.loads_multiple(data))
        assert docs == [{"a": 1}, {"b": 2}, {"c": 3}]
