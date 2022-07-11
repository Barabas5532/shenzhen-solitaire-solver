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


# %%
pattern = cv2.imread('images/GREEN_2.png', cv2.IMREAD_GRAYSCALE)
game = cv2.imread('images/game.png', cv2.IMREAD_GRAYSCALE)

# %%
print(game.shape)
print(pattern.shape)
print(game.dtype)
print(pattern.dtype)

# %%
#c = signal.correlate2d(game, pattern)
# https://docs.opencv.org/3.4/d4/dc6/tutorial_py_template_matching.html
c = cv2.matchTemplate(game, pattern, cv2.TM_SQDIFF_NORMED)
print(c.shape)
print(c.dtype)
print(f'min: {np.min(c)} max: {np.max(c)}')

# %%
threshold = 0.01
matches = np.where(c < threshold)

print(f'found {len(matches)} copies of pattern')
print(len(matches[0]))

h, w = pattern.shape
for pt in zip(*matches[::-1]):
    pass
    cv2.rectangle(c, pt, (pt[0] + w, pt[1] + h), 1, 2)

# %%
args = {'cmap':'gray', 'vmin': 0, 'vmax': 255}

plt.subplot(3, 1, 1)
plt.imshow(c)

plt.subplot(3, 1, 2)
plt.imshow(game, **args)

plt.subplot(3, 1, 3)
plt.imshow(pattern, **args)

plt.show()

# %%
