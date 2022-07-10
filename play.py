from solitaire import Game, GameState, Card, Suit

import resource
import sys

# 512 MB stack, 1 million recursive calls
resource.setrlimit(resource.RLIMIT_STACK, (512 * 2**10 * 2**10, -1))
sys.setrecursionlimit(10**6)

# From https://shenzhen-io.fandom.com/wiki/Shenzhen_Solitaire
# https://shenzhen-io.fandom.com/wiki/File:Solitaire.png
column0 = [
    Card(Suit.BLACK, None),
    Card(Suit.RED, None),
    Card(Suit.BLACK, 7),
    Card(Suit.RED, 7),
    Card(Suit.BLACK, 6),
]

column1 = [
    Card(Suit.RED, None),
    Card(Suit.RED, 9),
    Card(Suit.GREEN, 8),
    Card(Suit.GREEN, 7),
    Card(Suit.RED, 4),
]

column2 = [
    Card(Suit.GREEN, 2),
    Card(Suit.BLACK, 3),
    Card(Suit.BLACK, 5),
    Card(Suit.RED, 5),
    Card(Suit.GREEN, 3),
]

column3 = [
    Card(Suit.GREEN, 1),
    Card(Suit.RED, None),
    Card(Suit.SPECIAL, 1),
    Card(Suit.RED, 1),
    Card(Suit.GREEN, 6),
]

column4 = [
    Card(Suit.GREEN, 4),
    Card(Suit.RED, 8),
    Card(Suit.RED, 2),
    Card(Suit.RED, 6),
    Card(Suit.BLACK, None),
]

column5 = [
    Card(Suit.GREEN, 5),
    Card(Suit.BLACK, 4),
    Card(Suit.RED, None),
    Card(Suit.BLACK, 1),
    Card(Suit.BLACK, 8),
]

column6 = [
    Card(Suit.GREEN, None),
    Card(Suit.RED, 3),
    Card(Suit.GREEN, None),
    Card(Suit.BLACK, 2),
    Card(Suit.GREEN, None),
]

column7 = [
    Card(Suit.BLACK, 9),
    Card(Suit.BLACK, None),
    Card(Suit.GREEN, None),
    Card(Suit.GREEN, 9),
    Card(Suit.BLACK, None),
]

columns = (
    column0,
    column1,
    column2,
    column3,
    column4,
    column5,
    column6,
    column7,
)

states = [
    GameState(columns),
    GameState(
        (
            [
                Card(Suit.GREEN, 8),
                Card(Suit.BLACK, 1),
                Card(Suit.GREEN, None),
                Card(Suit.RED, 6),
                Card(Suit.BLACK, None),
            ],
            [
                Card(Suit.RED, 1),
                Card(Suit.RED, None),
                Card(Suit.RED, 4),
                Card(Suit.BLACK, None),
                Card(Suit.RED, 3),
            ],
            [
                Card(Suit.BLACK, 9),
                Card(Suit.BLACK, 6),
                Card(Suit.RED, None),
                Card(Suit.BLACK, 8),
                Card(Suit.BLACK, 2),
            ],
            [
                Card(Suit.GREEN, None),
                Card(Suit.GREEN, 3),
                Card(Suit.GREEN, 7),
                Card(Suit.SPECIAL, 1),
                Card(Suit.BLACK, 4),
            ],
            [
                Card(Suit.GREEN, 5),
                Card(Suit.RED, None),
                Card(Suit.GREEN, 6),
                Card(Suit.BLACK, 3),
                Card(Suit.RED, 8),
            ],
            [
                Card(Suit.GREEN, None),
                Card(Suit.GREEN, None),
                Card(Suit.GREEN, 2),
                Card(Suit.RED, 7),
                Card(Suit.RED, 9),
            ],
            [
                Card(Suit.RED, 5),
                Card(Suit.RED, None),
                Card(Suit.BLACK, 5),
                Card(Suit.GREEN, 4),
                Card(Suit.BLACK, None),
            ],
            [
                Card(Suit.BLACK, 7),
                Card(Suit.GREEN, 1),
                Card(Suit.RED, 2),
                Card(Suit.GREEN, 9),
                Card(Suit.BLACK, None),
            ],
        ),
    ),
    GameState(
        (
            [
                Card(Suit.BLACK, 1),
                Card(Suit.GREEN, 4),
                Card(Suit.BLACK, 5),
                Card(Suit.RED, None),
                Card(Suit.RED, 7),
            ],
            [
                Card(Suit.RED, 4),
                Card(Suit.BLACK, 4),
                Card(Suit.GREEN, 6),
                Card(Suit.GREEN, None),
                Card(Suit.GREEN, 3),
            ],
            [
                Card(Suit.RED, None),
                Card(Suit.BLACK, 7),
                Card(Suit.RED, None),
                Card(Suit.SPECIAL, 1),
                Card(Suit.RED, 9),
            ],
            [
                Card(Suit.GREEN, None),
                Card(Suit.GREEN, 1),
                Card(Suit.GREEN, None),
                Card(Suit.GREEN, None),
                Card(Suit.RED, 3),
            ],
            [
                Card(Suit.BLACK, 3),
                Card(Suit.BLACK, 2),
                Card(Suit.GREEN, 5),
                Card(Suit.RED, 1),
                Card(Suit.GREEN, 7),
            ],
            [
                Card(Suit.BLACK, None),
                Card(Suit.RED, 8),
                Card(Suit.RED, 2),
                Card(Suit.GREEN, 2),
                Card(Suit.GREEN, 8),
            ],
            [
                Card(Suit.RED, None),
                Card(Suit.GREEN, 9),
                Card(Suit.BLACK, None),
                Card(Suit.BLACK, 9),
                Card(Suit.RED, 5),
            ],
            [
                Card(Suit.BLACK, None),
                Card(Suit.BLACK, None),
                Card(Suit.RED, 6),
                Card(Suit.BLACK, 8),
                Card(Suit.BLACK, 6),
            ],
        ),
    ),
]
print(states)

for i, state in enumerate(states):
    game = Game()
    solution = game.play(state)

    assert solution is not None

    print(f"solution {i} length {len(solution)}")

    with open(f"solution{i}.txt", "w") as f:
        f.write(f"{solution}")
