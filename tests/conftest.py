import pytest
import numpy as np


@pytest.fixture
def insert_costs_a():
    insert_costs = {
        'D': 1.5
    }
    return insert_costs

@pytest.fixture
def insert_costs_b():
    insert_costs = np.ones(128, dtype=np.float64)  # make an array of all 1's of size 128, the number of ASCII characters
    insert_costs[ord('D')] = 1.5  # make inserting the character 'D' have cost 1.5 (instead of 1)
    return insert_costs


@pytest.fixture
def long_string_s1():
    return 'BANANA' * 100


@pytest.fixture
def long_string_s2():
    return 'BANDANA' * 100