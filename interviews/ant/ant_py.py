from PIL import Image

WIDTH = 1024
HEIGHT = 1024

UP = 0
RIGHT = 1
DOWN = 2
LEFT = 3

class Ant:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.direction = UP
    
    def step(self, grid):
        if grid[self.y][self.x]:  # On black cell
            self.direction = (self.direction - 1) % 4  # Rotate by 90 degree on the right
            grid[self.y][self.x] = 0  # Invertion
        else:  # On white cell
            self.direction = (self.direction + 1) % 4  # Rotate by 90 degree on the left
            grid[self.y][self.x] = 1  # Invertion
        
        if self.direction == UP:
            self.y -= 1
        elif self.direction == RIGHT:
            self.x += 1
        elif self.direction == DOWN:
            self.y += 1
        elif self.direction == LEFT:
            self.x -= 1

def main():
    grid = [[0 for _ in range(WIDTH)] for _ in range(HEIGHT)]
    ant = Ant(WIDTH // 2, HEIGHT // 2)
    
    while 0 <= ant.x < WIDTH and 0 <= ant.y < HEIGHT:
        ant.step(grid)
    
    img = Image.new('1', (WIDTH, HEIGHT), 'white')
    pixels = img.load()

    for y in range(HEIGHT):
        for x in range(WIDTH):
            if grid[y][x]:
                pixels[x, y] = 0  # black
    
    img.save('ant_path_py.png')

    black_cells_count = sum(sum(row) for row in grid)
    print(f"Black cells cnt: {black_cells_count}")

if __name__ == "__main__":
    main()
