import cv2
import solitaire

from image_processing.card_position_processing import get_state_from_image

state = get_state_from_image(cv2.imread('game.png'))
print(state)

game = solitaire.Game()
solution = game.play(state)
print(f'Found solution with {len(solution)} moves')

with open(f"solution.txt", "w") as f:
    f.write(f"{solution}")
