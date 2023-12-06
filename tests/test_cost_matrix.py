from distance_schmistance import cost_matrix

def test_cost_matrix():
    assert cost_matrix(1, {"!": 2})
    assert cost_matrix(2, {"!": {"?": 3}})