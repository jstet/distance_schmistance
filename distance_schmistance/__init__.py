from distance_schmistance._distance_schmistance import _wagner_fischer

__version__ = "0.1.0"

def cost_matrix(dim: int, dct: dict, code_dct: dict = None) -> list:
    """Generate a matrix of costs.

    Params:
    ------
    dim:
        whether the inputted dictionary is flat (delete costs or insert costs) or nestes (substitute costs)
    dct:
        the dictionary containing the costs
    code_dct:
        Optionally input a code dictionary if the keys of the cost dictionary are not encoded but the sequences are 

    Returns
    -------
    A  128 or 128 by 128 long 1 or 2 dim list. The indexes correspond to ASCII codes and the values to costs.

    Usage:
    ------
    >>> cost_matrix(1, {"1": 2}, code_dct)
    """
    if dim == 1:
        # 1 dim matrix of ones (standard cost). Length is 128 because of ASCII (all possible labels)
        costs = [1.0] * 128
        if dct is None:
            return costs
        # for every item in cost dict
        for key, value in dct.items():
            # encode or not
            if code_dct is None:
                # ord converts a character to its asci value
                costs[ord(key)] = value
            else:
                costs[ord(code_dct[key])] = value
        return costs
    elif dim == 2:
        # 2 dim matrix of ones (standard cost).
        costs = [[1.0] * 128 for _ in range(128)]
        if dct is None:
            return costs
        # for every item in cost dict
        for key, value in dct.items():
            # for every item corresponding to top level key in cost dict
            for key2, value2 in value.items():
                # encode or not
                if code_dct is None:
                    costs[ord(key)][ord(key2)] = value2
                else:
                    costs[ord(code_dct[key])][ord(code_dct[key2])] = value2
    return costs

def edit_distance(s1, s2, insert_costs = None, delete_costs = None, substitute_costs = None):
    delete_costs = cost_matrix(1, delete_costs)
    insert_costs = cost_matrix(1, insert_costs)
    substitute_costs = cost_matrix(2, substitute_costs)
    return _wagner_fischer(s1, s2, insert_costs=insert_costs, delete_costs=delete_costs, substitute_costs=substitute_costs)



