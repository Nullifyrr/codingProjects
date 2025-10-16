def calc(x, y, o):
    if o == "+":
        print(f"{x} + {y} = {x + y}")
    elif o == "-":
        print(f"{x} - {y} = {x - y}")
    elif o == "*":
        print(f"{x} * {y} = {x * y}")
    elif o == "/":
        print(f"{x} / {y} = {x / y}")
    elif o == "^":
        print(f"{x} ^ {y} = {x ** y}")
    else:
        print("ERR: Invalid operation!")

x = 0 # Declare variables
y = 0
o = ""
print("Calculator!") # Show project name

print("What would you like the first number to be?")
x = float(input(""))

print("What about the operation?")
o = input("")

print("Finally, the second number?")
y = float(input(""))

calc(x,y,o)