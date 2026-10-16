# Copyright (c) 2025-2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from pathlib import Path

import pytest

from pyavd_utils.schema_store import init_store_from_file
from pyavd_utils_gen.schema_store import compile_schema_archive

TEST_SCHEMA_STORE = Path(__file__).parent / "schemas" / "schemas.json"


@pytest.fixture(scope="session")
def init_store(tmp_path_factory: pytest.TempPathFactory) -> None:
    schema_dir = tmp_path_factory.mktemp("schema_store")
    archive_file = schema_dir / "schemas.rkyv"
    compile_schema_archive(TEST_SCHEMA_STORE, archive_file)

    init_store_from_file(archive_file)
