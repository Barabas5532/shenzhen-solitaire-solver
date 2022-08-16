import unittest

import auto_solve
from solitaire import Card, Suit


class ScreenCoordinatesTest(unittest.TestCase):
    def test_top_right_storage_special(self) -> None:
        # Special suit is always in the same place
        uut = auto_solve.ScreenCoordinates()
        result = uut.get_top_right_storage(Card(Suit.SPECIAL, 1))
        self.assertEqual((1030, 240), result)

    def test_top_right_storage_other(self) -> None:
        pass
        # uut = auto_solve.ScreenCoordinates()
        # result = uut.get_top_right_storage(Card(Suit.RED, 1))

        # expect call to pyautogui locateallonscreen with the top right region,
        # where the first result of that is used as the position of the
        # foundation to use for this suit


unittest.main()
