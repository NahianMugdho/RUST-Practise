import turtle
import time

# screen setup
screen = turtle.Screen()
screen.title("Fibonacci Recursion Animation")

t = turtle.Turtle()
t.speed(0)
t.hideturtle()

def draw_text(x, y, text):
    t.penup()
    t.goto(x, y)
    t.write(text, align="center", font=("Arial", 10, "bold"))

def fib_tree(x, pos_x, pos_y, offset):
    # node লিখা
    draw_text(pos_x, pos_y, f"fib({x})")
    time.sleep(0.5)

    # 👉 BASE CASE (তোমার main focus)
    if x < 2:
        draw_text(pos_x, pos_y - 15, f"= {x}")
        return

    # left branch
    t.penup()
    t.goto(pos_x, pos_y)
    t.pendown()
    t.goto(pos_x - offset, pos_y - 60)

    # right branch
    t.penup()
    t.goto(pos_x, pos_y)
    t.pendown()
    t.goto(pos_x + offset, pos_y - 60)

    # recursive calls
    fib_tree(x - 1, pos_x - offset, pos_y - 60, offset / 2)
    fib_tree(x - 2, pos_x + offset, pos_y - 60, offset / 2)


# run animation
fib_tree(5, 0, 250, 120)

turtle.done()