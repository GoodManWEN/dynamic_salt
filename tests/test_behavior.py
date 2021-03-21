import os , sys
sys.path.append(os.getcwd())
import pytest
import time
from dsalt import *
from utils import rand_char_generator

@pytest.mark.asyncio
async def test_speed():
    test_time = 1_000_000
    st_time = time.time()
    for i in range(st_time):
        ...
    ed_time = time.time()
    blank_time = ed_time - st_time
    st_time = time.time()
    for _ in range(st_time):
        salt_core(b"")
    ed_time = time.time()
    print('\n',f"Empty array takes time: {round((ed_time - st_time) * 1000,2)} ns")
    st_time = time.time()
    for _ in range(st_time):
        salt_core(b"Hello world")
    ed_time = time.time()
    print('\n',f"Default array takes time: {round((ed_time - st_time) * 1000,2)} ns")


@pytest.mark.asyncio
async def test_stable():
    for _ in range(100_000):
        rchar = rand_char_generator()
        res = salt_core(rchar)
        assert len(rchar) < len(res) <= (rchar * 2)
