import pytest
import numpy as np
from distance_schmistance import cost_matrix


@pytest.fixture
def insert_costs_a():
    insert_costs = {
        'D': 1.5
    }
    insert_costs = cost_matrix(1, insert_costs)
    return insert_costs

@pytest.fixture
def substitute_costs_a():
    substitute_costs = None
    substitute_costs = cost_matrix(2, substitute_costs)
    return substitute_costs

@pytest.fixture
def delete_costs_a():
    delete_costs = None
    delete_costs = cost_matrix(1, delete_costs)
    return delete_costs


@pytest.fixture
def insert_costs_b():
    insert_costs = np.ones(128, dtype=np.float64)  # make an array of all 1's of size 128, the number of ASCII characters
    insert_costs[ord('D')] = 1.5  # make inserting the character 'D' have cost 1.5 (instead of 1)
    return insert_costs


@pytest.fixture
def long_string_s1():
    return 'BANANA' * 50


@pytest.fixture
def long_string_s2():
    return 'BANDANA' * 50