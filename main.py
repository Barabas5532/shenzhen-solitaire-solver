import cv2  # type: ignore
import solitaire

from image_processing.card_position_processing import get_state_from_image

state = get_state_from_image(cv2.imread("game.png"))
print(state)

game = solitaire.Game()
solution = game.play(state)

assert solution

print(f"Found solution with {len(solution)} moves")

with open("solution.txt", "w") as f:
    f.write(f"{solution}")
