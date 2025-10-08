# SPDX-License-Identifier: (Apache-2.0 OR MIT)
import hashlib
from json import loads as json_loads

import orjson
import pytest

from .util import read_fixture


def json_loads_ndjson(data: bytes):
    for line in data.split(b"\n"):
        if line:
            yield json_loads(line)


@pytest.mark.parametrize("loader", [orjson.loads_multiple, json_loads_ndjson])
def test_load_multiple(benchmark, loader):
    # curl https://microsoftedge.github.io/Demos/json-dummy-data/5MB.json | jq -c '.[]' | xz -9 - > data/5MB.ndjson.xz
    data = read_fixture(f"5MB.ndjson.xz")

    def t():
        id_hasher = hashlib.sha256()
        for n, x in enumerate(loader(data)):
            assert set(x) == {"name", "language", "id", "bio", "version"}
            id_hasher.update(x["id"].encode())
        return (n, id_hasher.hexdigest())

    # These verify that both loaders load pretty much the same data.
    assert benchmark(t) == (15839, '266b485545f966cb708b20ce98991f032e2ee2e8fcc6d0499a8deb49aafe5fba')
