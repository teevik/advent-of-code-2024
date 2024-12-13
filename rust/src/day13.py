from sympy import *

a_presses, b_presses, a_x, a_y, b_x, b_y, p_x, p_y = symbols('a_presses b_presses a_x a_y b_x b_y p_x p_y')

eq1 = Eq(a_x * a_presses + b_x * b_presses, p_x)
eq2 = Eq(a_y * a_presses + b_y * b_presses, p_y)

# Solve equations
sol = solve([eq1, eq2], (a_presses, b_presses))
print(sol)
