import matplotlib.pyplot as plt
from ast import literal_eval as make_tuple
from sympy import Function, Symbol, Lambda
from sympy.parsing.sympy_parser import parse_expr, auto_symbol
import numpy as np
import os

# Run the Rust calculations
os.system("cargo run")

# Retrieve the Rust calculations
file = open("r_roots.txt", "r")

# Parse our Rust calculations
r_roots_str = file.read()
r_roots_str_list = r_roots_str.splitlines()


# Make a list of tuple of rational roots
r_roots_tup_lis = [make_tuple(tuple_str) for tuple_str in r_roots_str_list]

# Flatten the above list (We use this as the domain)
X = [root for tuple in r_roots_tup_lis for root in tuple]


print("X = ", X)

# Get the math function from the user as a string
func = input("Enter a polynomial function\n e.g. ax^2 + bx + c: ")

# Parse the user given math function  
x = Symbol("x")
f = Lambda(x, parse_expr(func, transformations="all"))

# Calculate the values when the potential roots plugged in 
Y = [f(root) for root in X]

print("Y = ", Y, end="\n")

# Determine which, if any, are actual solutions 
roots_actual = [root for root in X if f(root) == 0.0]
if len(roots_actual) != 0:
    print(f"Rational roots which work: {roots_actual}")
else:
    print("Sorry, no rational roots!")

plt.style.use('ggplot')
plt.xlabel("x-axis")
plt.ylabel("y-axis")
plt.title(f"Constrained Rational Roots for y = {parse_expr(func, transformations='all')}")

plt.stem(
    np.asarray(X, dtype=float), np.asarray(Y, dtype=float), use_line_collection=True
)
# plt.scatter(X, Y)
plt.show()
