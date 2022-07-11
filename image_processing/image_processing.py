# %% [markdown]
# # Image processing notebook
#
# Playing around with image processing algorithms for generating the initial game state from a screenshot.
#
# https://docs.opencv.org/3.4/d4/dc6/tutorial_py_template_matching.html

# %%
# %matplotlib widget

import numpy as np
import scipy as sp
import matplotlib.pyplot as plt
from scipy import ndimage
from scipy import signal
import cv2
from pprint import pprint


# %%
# %load_ext autoreload
# %autoreload 2
from image_list import *

# %%
card_image_names = CardImageNames('images')
card_image_names.get_name(Card(Suit.BLACK, None))


# %%
def bgr_to_rgb(image):
    return cv2.cvtColor(image, cv2.COLOR_BGR2RGB)


# %%
game = cv2.imread('images/game.png')
print(game.shape)
print(game.dtype)


# %%
def find_matches(image, template):
    c = cv2.matchTemplate(image, template, cv2.TM_SQDIFF_NORMED)

    threshold = 0.01
    matches = np.where(c < threshold)

    assert len(matches[0]) == len(matches[1])
    return matches


# %%
pattern = cv2.imread(card_image_names.get_name(Card(Suit.GREEN, 5)))
print(pattern.shape)
print(pattern.dtype)

# %%
matches = find_matches(game, pattern)

print(f'found {len(matches[0])} copies of pattern')

annotated = game.copy()
h, w, _ = pattern.shape
for pt in zip(*matches[::-1]):
    pass
    cv2.rectangle(annotated, pt, (pt[0] + w, pt[1] + h), (255, 0, 255), 2)

# %%
plt.close()
plt.figure()
plt.subplot(3, 1, 1)
plt.imshow(bgr_to_rgb(annotated))

plt.subplot(3, 1, 2)
plt.imshow(bgr_to_rgb(game))

plt.subplot(3, 1, 3)
plt.imshow(bgr_to_rgb(pattern))

plt.show()

# %% [markdown]
# # Sanity check
#
# There should be 1 match for any card that is not a dragon, and 4 matches for each dragon

# %%
cards = [Card(Suit.SPECIAL, 1)]
for suit in [Suit.RED, Suit.GREEN, Suit.BLACK]:
    for value in [*range(1, 9 + 1), None]:
        cards.append(Card(suit, value))

card_position = {}
for card in cards:
    pattern = cv2.imread(card_image_names.get_name(card))
    matches = find_matches(game, pattern)
    print(f'Card: {card}, matches: {matches}')
    assert len(matches[0]) == (4 if card.is_dragon() else 1)
    
    card_position[card] = list(zip(*matches[::-1]))

# %% [markdown]
# Now we have the positions of all the cards. We need to cluster the results into each column. We can do this easily by adding a 5 to 10 pixel threshold on the x coordinate and collecting similar x coordinates together.
#
# After this we can just sort by the y position to get the cards in the correct order for the column.

# %%
pprint(card_position)

# %%
