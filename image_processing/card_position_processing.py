from dataclasses import dataclass, replace
from operator import attrgetter
import cv2
import os
import numpy as np

from image_processing.image_list import *
from solitaire import *

dir_path = os.path.dirname(os.path.realpath(__file__))

@dataclass(frozen=True)
class CardWithPosition:
    card: Card
    position: tuple[int, int]

card_image_names = CardImageNames(dir_path + '/images')

def _find_matches(image, template):
    c = cv2.matchTemplate(image, template, cv2.TM_SQDIFF_NORMED)

    threshold = 0.01
    matches = np.where(c < threshold)

    assert len(matches[0]) == len(matches[1])
    return matches

def get_state_from_image(game) -> GameState:
    cards = _get_list_of_all_cards()
    card_position = _get_position_of_each_card(game, cards)
    card_list = _get_card_position_list(card_position)
    
    return GameState(_sort_cards_to_columns(card_list))
    
def _get_list_of_all_cards() -> list[Card]:
    cards = [Card(Suit.SPECIAL, 1)]
    for suit in [Suit.RED, Suit.GREEN, Suit.BLACK]:
        for value in [*range(1, 9 + 1), None]:
            cards.append(Card(suit, value))
    
    return cards

def _get_position_of_each_card(game, cards: list[Card]) -> tuple[int, int]:
    card_position = {}
    for card in cards:
        pattern = cv2.imread(card_image_names.get_name(card))
        matches = _find_matches(game, pattern)
        assert len(matches[0]) == (4 if card.is_dragon() else 1)

        card_position[card] = tuple(zip(*matches[::-1]))

    return card_position

def _get_card_position_list(card_position):
    card_list = []
    for card in card_position:
        for position in card_position[card]:
            card_list.append(CardWithPosition(card, position))
    return card_list

def _sort_cards_to_columns(card_list: list[CardWithPosition]) -> list[list[Card]]:
    # 8 columns with 5 cards each
    assert len(card_list) == (8 * 5)

    # round the x coordinate to a multiple of 100
    round_x = lambda c: replace(c, position=(c.position[0]//100 * 100, c.position[1]))
    card_list = list(map(round_x, card_list))
    # Sort the cards by x and y position
    card_list = sorted(card_list, key=attrgetter('position'))
    
    N = 5 # cards in each column
    columns = [card_list[i:i+N] for i in range(0, len(card_list), N)]
    for column in columns:
        column[:] = list(map(attrgetter('card'), column))
    return columns