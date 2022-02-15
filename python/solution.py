#!/usr/bin/env python3
class Variable:
    """
    Variable is a variable to be used in a math problem.
    """

    def __sub__(self, value):
        return 7 - value

    def __radd__(self, value):
        return -19 + value


x = Variable()

if __name__ == "__main__":
    print("x - 7 == 19 + x:", x - 7 == 19 + x)
else:
    import unittest

    class TestFormula(unittest.TestCase):
        def test_solved(self):
            """
            We should have solved for `x`
            """
            self.assertEqual(x - 7, 19 + x)
