from distance_schmistance import edit_distance

def test_edit_distance():
    assert edit_distance('distance', 'schmistance', substitute_costs = {"d": {"s": 0.1}}) == 3.1