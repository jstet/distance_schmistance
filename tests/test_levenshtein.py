from distance_schmistance import levenshtein

def test_levenshtein():
    assert levenshtein("distance", "schmistance") == 4