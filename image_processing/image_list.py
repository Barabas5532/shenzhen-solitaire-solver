import sys
from typing import Optional

sys.path.append('..')

from solitaire import Card, Suit

class CardImageNames:
    _suit_names = {
        Suit.SPECIAL: 'SPECIAL',
        Suit.RED: 'RED',
        Suit.GREEN: 'GREEN',
        Suit.BLACK: 'BLACK',
    }

    def __init__(self, prefix: str) -> None:
        self.prefix = prefix

    def _value_name(self, value: Optional[int]) -> str:
        if value is None:
            return 'DRAGON'

        return str(value)

    def get_name(self, card: Card) -> str:
        return (
                f'{self.prefix}/{self._suit_names[card.suit]}_'
                f'{self._value_name(card.value)}.png' )
