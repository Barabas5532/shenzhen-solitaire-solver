import os
from dataclasses import dataclass

import cv2  # type: ignore
import numpy as np

from image_processing.image_list import CardImageNames
from solitaire import Card, Columns, GameState, Suit

dir_path = os.path.dirname(os.path.realpath(__file__))


@dataclass(frozen=True)
class CardWithPosition:
    card: Card
    position: tuple[int, int]


card_image_names = CardImageNames(dir_path + "/images")


def _find_matches(
    image: np.ndarray,
    template: np.ndarray,
) -> tuple[list[int], list[int]]:
    c = cv2.matchTemplate(image, template, cv2.TM_SQDIFF_NORMED)

    threshold = 0.01
    matches = np.where(c < threshold)

    assert len(matches[0]) == len(matches[1])
    assert len(matches) == 2

    m0: list[int] = matches[0].tolist()
    m1: list[int] = matches[1].tolist()

    return (m0, m1)


def get_state_from_image(game: np.ndarray) -> GameState:
    cards = _get_list_of_all_cards()
    card_position = _get_position_of_each_card(game, cards)
    card_list = _get_card_position_list(card_position)
    top_right = _get_top_right_cards(card_position)

    return GameState(_sort_cards_to_columns(card_list), [], top_right)


def _get_list_of_all_cards() -> list[Card]:
    cards = [Card(Suit.SPECIAL, 1)]
    for suit in [Suit.RED, Suit.GREEN, Suit.BLACK]:
        for value in [*range(1, 9 + 1), None]:
            cards.append(Card(suit, value))

    return cards


def _get_position_of_each_card(
    game: np.ndarray,
    cards: list[Card],
) -> dict[Card, tuple[tuple[int, int], ...]]:
    card_position = {}
    for card in cards:
        pattern = cv2.imread(card_image_names.get_name(card))
        matches = _find_matches(game, pattern)
        if card.is_dragon():
            assert len(matches[0]) == 4
        else:
            assert len(matches[0]) <= 1

        card_position[card] = tuple(zip(*matches[::-1]))

    return card_position


def _get_card_position_list(
    card_position: dict[Card, tuple[tuple[int, int], ...]],
) -> list[CardWithPosition]:
    card_list = []
    for card in card_position:
        for position in card_position[card]:
            card_list.append(CardWithPosition(card, position))
    return card_list


def _get_top_right_cards(
    card_positions: dict[Card, tuple[tuple[int, int], ...]]
) -> list:
    out = []
    if not len(card_positions[Card(Suit.SPECIAL, 1)]):
        out.append(1)
    else:
        out.append(0)

    for suit in [Suit.RED, Suit.GREEN, Suit.BLACK]:
        for value in reversed(range(1, 9 + 1)):
            if not len(card_positions[Card(suit, value)]):
                out.append(value)
                break

            if value == 1:
                out.append(0)

    assert len(out) == 4
    assert out[0] <= 1

    return out


def _sort_cards_to_columns(
    card_list: list[CardWithPosition],
) -> Columns:
    # 8 columns with up to 5 cards each
    assert len(card_list) <= (8 * 5)

    columns: list[list[CardWithPosition]] = [[card_list[0]]]

    for card in card_list[1:]:
        for i, column in enumerate(columns):
            if abs(column[0].position[0] - card.position[0]) < 100:
                column.append(card)
                break

            if i == len(columns) - 1:
                columns.append([card])
                break

    assert len(columns) == 8

    for column in columns:
        assert len(column) >= 1

    # At this point, columns have been created, each containing the cards from
    # that column. The cards within the columns are not sorted. The columns are
    # also not sorted by their x position.

    for column in columns:
        column[:] = sorted(
            column,
            key=lambda card: card.position[1],
        )

    columns = sorted(columns, key=lambda column: column[0].position[0])

    out: list[list[Card]] = []
    for column in columns:
        out.append([card_pos.card for card_pos in column])

    assert len(out) == 8

    return (
        out[0],
        out[1],
        out[2],
        out[3],
        out[4],
        out[5],
        out[6],
        out[7],
    )


if __name__ == "__main__":
    import unittest

    class ImageProcessingTest(unittest.TestCase):
        def test_unsolved(self) -> None:
            game_image = cv2.imread(
                dir_path + "/test_images/unsolved_start.png"
            )

            expected = GameState(
                (
                    [
                        Card(Suit.GREEN, 8),
                        Card(Suit.GREEN, 6),
                        Card(Suit.RED, None),
                        Card(Suit.GREEN, 2),
                        Card(Suit.RED, None),
                    ],
                    [
                        Card(Suit.RED, 3),
                        Card(Suit.SPECIAL, 1),
                        Card(Suit.RED, 5),
                        Card(Suit.RED, 4),
                        Card(Suit.BLACK, 3),
                    ],
                    [
                        Card(Suit.BLACK, 1),
                        Card(Suit.RED, 2),
                        Card(Suit.BLACK, 7),
                        Card(Suit.BLACK, None),
                        Card(Suit.RED, None),
                    ],
                    [
                        Card(Suit.BLACK, None),
                        Card(Suit.GREEN, 1),
                        Card(Suit.BLACK, None),
                        Card(Suit.RED, 7),
                        Card(Suit.BLACK, 8),
                    ],
                    [
                        Card(Suit.BLACK, 5),
                        Card(Suit.GREEN, 5),
                        Card(Suit.GREEN, None),
                        Card(Suit.RED, None),
                        Card(Suit.BLACK, None),
                    ],
                    [
                        Card(Suit.RED, 8),
                        Card(Suit.GREEN, None),
                        Card(Suit.RED, 1),
                        Card(Suit.GREEN, None),
                        Card(Suit.BLACK, 4),
                    ],
                    [
                        Card(Suit.GREEN, None),
                        Card(Suit.BLACK, 6),
                        Card(Suit.RED, 9),
                        Card(Suit.GREEN, 3),
                        Card(Suit.RED, 6),
                    ],
                    [
                        Card(Suit.GREEN, 9),
                        Card(Suit.GREEN, 7),
                        Card(Suit.BLACK, 2),
                        Card(Suit.BLACK, 9),
                        Card(Suit.GREEN, 4),
                    ],
                )
            )

            actual = get_state_from_image(game_image)

            self.assertEqual(expected, actual)

        def test_partially_solved(self) -> None:
            game_image = cv2.imread(
                dir_path + "/test_images/partially_solved_start.png"
            )

            expected = GameState(
                (
                    [
                        Card(Suit.BLACK, None),
                        Card(Suit.RED, None),
                        Card(Suit.GREEN, 2),
                        Card(Suit.GREEN, 7),
                        Card(Suit.RED, 5),
                    ],
                    [
                        Card(Suit.RED, None),
                        Card(Suit.GREEN, 9),
                        Card(Suit.BLACK, 7),
                        Card(Suit.GREEN, 5),
                    ],
                    [
                        Card(Suit.GREEN, None),
                        Card(Suit.BLACK, None),
                        Card(Suit.RED, None),
                        Card(Suit.BLACK, None),
                        Card(Suit.GREEN, 6),
                    ],
                    [
                        Card(Suit.BLACK, 5),
                        Card(Suit.RED, 9),
                        Card(Suit.RED, 7),
                        Card(Suit.BLACK, 4),
                        Card(Suit.BLACK, None),
                    ],
                    [
                        Card(Suit.BLACK, 6),
                        Card(Suit.GREEN, 3),
                        Card(Suit.RED, 2),
                        Card(Suit.BLACK, 3),
                        Card(Suit.GREEN, None),
                    ],
                    [
                        Card(Suit.RED, 3),
                        Card(Suit.GREEN, None),
                        Card(Suit.BLACK, 8),
                        Card(Suit.RED, 6),
                    ],
                    [
                        Card(Suit.GREEN, 4),
                        Card(Suit.RED, 1),
                        Card(Suit.GREEN, None),
                    ],
                    [
                        Card(Suit.BLACK, 9),
                        Card(Suit.RED, 8),
                        Card(Suit.RED, None),
                        Card(Suit.GREEN, 8),
                        Card(Suit.RED, 4),
                    ],
                ),
                [],
                [1, 0, 1, 2],
            )

            actual = get_state_from_image(game_image)

            self.assertEqual(expected, actual)

    unittest.main()
