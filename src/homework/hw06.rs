def draw_triangle(height):
    for i in range(height):
        print(' ' * (height - i - 1) + '*' * (2 * i + 1))

def draw_tree(num_triangles):
    for height in range(1, num_triangles + 1):
        draw_triangle(height)

# Кількість трикутників
num_triangles = 5  # Ви можете змінити цю кількість
draw_tree(num_triangles)
