from distance_schmistance import edit_distance
from weighted_levenshtein import lev
import pytest

@pytest.mark.benchmark_schmistance_short
def test_benchmark_schmistance_short(benchmark, insert_costs_a):
    # benchmark something
    result = benchmark(edit_distance, 'BANANA', 'BANDANA', insert_costs_a)

    # Extra code, to verify that the run completed correctly.
    # Sometimes you may want to check the result, fast functions
    # are no good if they return incorrect results :-)
    assert result == 1.5


@pytest.mark.benchmark_weighted_levenshtein_short
def test_benchmark_weighted_levenshtein_short(benchmark, insert_costs_b):
    # benchmark something
    result = benchmark(lev, 'BANANA', 'BANDANA', insert_costs_b)

    # Extra code, to verify that the run completed correctly.
    # Sometimes you may want to check the result, fast functions
    # are no good if they return incorrect results :-)
    assert result == 1.5

@pytest.mark.benchmark_schmistance_long
def test_benchmark_schmistance_long(benchmark, insert_costs_a, long_string_s1, long_string_s2):
    # benchmark something
    result = benchmark(edit_distance, 'BANANA', 'BANDANA', insert_costs_a)

    # Extra code, to verify that the run completed correctly.
    # Sometimes you may want to check the result, fast functions
    # are no good if they return incorrect results :-)
    assert result == 1.5


@pytest.mark.benchmark_weighted_levenshtein_long
def test_benchmark_weighted_levenshtein_long(benchmark, insert_costs_b, long_string_s1, long_string_s2):
    # benchmark something
    result = benchmark(lev, 'BANANA', 'BANDANA', insert_costs_b)

    # Extra code, to verify that the run completed correctly.
    # Sometimes you may want to check the result, fast functions
    # are no good if they return incorrect results :-)
    assert result == 1.5