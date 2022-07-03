import itertools
import copy

SPECIAL = 0
RED = 1
GREEN = 2
BLACK = 3
FACE_DOWN = 4


class Card:
    colors = ["🆒", "🟥", "🟩", "⬛"]

    def __init__(self, suit, value: int):
        self.suit = suit
        self.value = value

    def __str__(self):
        if self.suit is FACE_DOWN:
            return "xxx"
        return (
            f"{self.colors[self.suit]}"
            f"{self.value if self.value is not None else 'x'}"
        )

    # quick hack to make printing an array of card readable
    def __repr__(self):
        return self.__str__()

    def __hash__(self):
        return hash(frozenset((self.suit, self.value)))

    def __eq__(self, other):
        return (self.suit, self.value) == (other.suit, other.value)


TOP_LEFT = 0
CENTRE = 1


class CardPosition:
    def __init__(self, location, index: int):
        self.location = location
        self.index = index


class GameState:
    def __init__(
        self,
        columns,
        top_left_storage=[],
        top_right_storage=[0 for _ in range(4)],
    ):
        assert list == type(top_left_storage)
        assert len(top_left_storage) <= 3

        # scratch pad to temporarily store cards
        # a space is lost when dragons are stacked here, represented by a
        # Card(FACE_DOWN, None)
        self.top_left_storage = top_left_storage
        # The aim of the game is to get all the cards stacked here
        self.top_right_storage = top_right_storage

        # The main play area, where all of the cards are placed at the start
        self.columns = columns

    # All the columns in the centre have no cards
    def is_solved(self):
        for column in self.columns:
            if len(column) != 0:
                return False

        # this is only possible if all the dragons have been collected, and the
        # top right cards all have value 9
        # This is just a sanity check to make sure the game has not in an
        # invalid state
        assert len(self.top_right_storage) == 4
        assert self.top_right_storage[0] == 1
        for i in range(1, 4):
            assert self.top_right_storage[i] == 9
        assert len(self.top_left_storage) == 3
        for i in range(3):
            assert self.top_left_storage[i].suit == FACE_DOWN

        return True

    # TODO SPECIAL card can always be moved to storage, it's hardcoded to have
    # value of 1 for now
    def _get_move_to_top_right_storage(self):
        for i, column in enumerate(self.columns):
            if len(column) == 0:
                continue
            card = column[-1]
            if (
                card.value == 1
                or card.value is not None
                and self.top_right_storage[card.suit] == card.value - 1
            ):
                return CardPosition(CENTRE, i)

        for i, card in enumerate(self.top_left_storage):
            assert card.value != 1
            if (
                card.value is not None
                and self.top_right_storage[card.suit] == card.value - 1
            ):
                return CardPosition(TOP_LEFT, i)

        return None

    def can_move_to_top_right_storage(self):
        return self._get_move_to_top_right_storage() is not None

    def move_to_top_right_storage(self):
        card_position = self._get_move_to_top_right_storage()
        assert card_position is not None
        column_to_move = card_position.index

        if card_position.location == CENTRE:
            card = self.columns[column_to_move].pop()
        elif card_position.location == TOP_LEFT:
            card = self.top_left_storage.pop(card_position.index)
        else:
            assert False

        self.top_right_storage[card.suit] = card.value

    def can_move_top_left_to_column(self, top_left_index, column_index):
        if top_left_index >= len(self.top_left_storage):
            return False

        card_to_move = self.top_left_storage[top_left_index]
        # Can't move collected dragons
        if card_to_move.suit is FACE_DOWN:
            return False

        # We are sure that the top left storage has a movable card now

        # moving to an empty column is always allowed
        if len(self.columns[column_index]) == 0:
            return True

        target_card = self.columns[column_index][-1]
        # can't move on top of dragon
        if target_card.value is None:
            return False

        return (
            card_to_move.suit != target_card.suit
            and card_to_move.value == target_card.value - 1
        )

    def move_top_left_to_column(self, top_left_index, column_index):
        self.columns[column_index].append(
            self.top_left_storage.pop(top_left_index)
        )

    def __str__(self):
        top_row = "========== GAME STATE =========\n"
        for i, card in enumerate(self.top_left_storage):
            top_row += str(card)
            top_row += " "

        for i in range(3 - len(self.top_left_storage) + 1):
            top_row += "    "

        for suit, value in enumerate(self.top_right_storage):
            if value == 0:
                top_row += "   "
            else:
                top_row += str(Card(suit, value))
            top_row += " "

        # transpopse rows and columns so we can print the cards in the layout
        # that matches the game
        transposed = list(
            map(list, itertools.zip_longest(*self.columns, fillvalue=None))
        )

        columns = ""
        for i, column in enumerate(transposed):
            for j, card in enumerate(column):
                if card is None:
                    columns += "   "
                else:
                    columns += str(card)
                columns += " "
            columns += "\n"

        return top_row + "\n" + columns

    def _tuple(self):
        return (
            tuple(self.top_left_storage),
            tuple([tuple(column) for column in self.columns]),
            tuple(self.top_right_storage),
        )

    def __eq__(self, other):
        return self._tuple() == other._tuple()

    def __hash__(self):
        return hash(self._tuple())


class Game:
    def __init__(self):
        self.seen_states = set()

    def already_seen(self, state):
        if state in self.seen_states:
            return True

        self.seen_states.add(state)
        return False

    def play(self, state: GameState):
        # the game forces us to move any cards to the top right storage if it's
        # a valid move
        #
        # This is also our base case, as the game is won when all cards have
        # been stacked here.
        #
        # The dragons might not all be collected yet, but that's fine, since if
        # we have all the numbered cards, the dragons are always free to
        # collect as the final move. This might also be automatic.
        #
        # TODO Does this also happen for cards in the top left storage?
        #      In any case, they would be collected to the top right as soon as
        #      they are moved away from the top left, there is no point keeping
        #      them there.
        #

        # If we have made a loop of moves, terminate. This prevents following
        # the cycle infinitely
        if self.already_seen(state):
            return None

        if state.is_solved():
            return [state]

        # The input state is owned by the caller, and we must not modify it
        state_copy = copy.deepcopy(state)

        if state_copy.can_move_to_top_right_storage():
            state_copy.move_to_top_right_storage()
            result = self.play(state_copy)
            if result is not None:
                return [state, *result]
            # We have to make this move, the game won't let us do anything
            # else. If it results in a losing game, then we need to backtrack
            return None

        # test out every possible move. The list of all moves are:
        # move a card out of the top left storage area to a column
        for top_left_index in range(3):
            for column_index in range(8):
                if state_copy.can_move_top_left_to_column(
                    top_left_index, column_index
                ):
                    state_copy.move_top_left_to_column(
                        top_left_index, column_index
                    )
                    result = self.play(state_copy)
                    if result is not None:
                        return [state, *result]
                    # else we keep looping to try all the possible moves

        # move a card from the centre to the storage area

        # collect dragons

        # move any set of cards from any column to any other column

        return None


if __name__ == "__main__":

    def debug_print():
        print(Card(suit=RED, value=5))
        print(Card(suit=BLACK, value=0))
        print(Card(suit=GREEN, value=None))
        print(Card(suit=SPECIAL, value=1))

        # From https://shenzhen-io.fandom.com/wiki/Shenzhen_Solitaire
        # https://shenzhen-io.fandom.com/wiki/File:Solitaire.png
        column0 = [
            Card(BLACK, None),
            Card(RED, None),
            Card(BLACK, 7),
            Card(RED, 7),
            Card(BLACK, 6),
        ]

        column1 = [
            Card(RED, None),
            Card(RED, 9),
            Card(GREEN, 8),
            Card(GREEN, 7),
            Card(RED, 4),
        ]

        column2 = [
            Card(GREEN, 2),
            Card(BLACK, 3),
            Card(BLACK, 5),
            Card(RED, 5),
            Card(GREEN, 3),
        ]

        column3 = [
            Card(GREEN, 1),
            Card(RED, None),
            Card(SPECIAL, 1),
            Card(RED, 1),
            Card(GREEN, 6),
        ]

        column4 = [
            Card(GREEN, 4),
            Card(RED, 8),
            Card(RED, 2),
            Card(RED, 6),
            Card(BLACK, None),
        ]

        column5 = [
            Card(GREEN, 5),
            Card(BLACK, 4),
            Card(RED, None),
            Card(BLACK, 1),
            Card(BLACK, 8),
        ]

        column6 = [
            Card(GREEN, None),
            Card(RED, 3),
            Card(GREEN, None),
            Card(BLACK, 2),
            Card(GREEN, None),
        ]

        column7 = [
            Card(BLACK, 9),
            Card(BLACK, None),
            Card(GREEN, None),
            Card(GREEN, 9),
            Card(BLACK, None),
        ]

        columns = [
            column0,
            column1,
            column2,
            column3,
            column4,
            column5,
            column6,
            column7,
        ]

        state = GameState(columns)
        print(state)

        # XXX: This is an invalid game state, for printing demo only
        progressed_columns = [
            [],
            column1[:2],
            column2,
            column3[:2],
            column4[:1],
            [],
            column6,
            column7[:2],
        ]
        progressed_state = GameState(
            progressed_columns, [Card(FACE_DOWN, None)], [1, 2, 3, 4]
        )
        print(progressed_state)

    import unittest

    class CardTest(unittest.TestCase):
        def test_hashable(self):
            a = Card(RED, 1)
            b = Card(RED, 1)
            c = Card(GREEN, 1)
            d = Card(RED, 2)

            self.assertFalse(a is b)
            self.assertTrue(a == b)
            self.assertTrue(hash(a) == hash(b))

            self.assertFalse(a == c)
            self.assertFalse(hash(a) == hash(c))

            self.assertFalse(a == d)
            self.assertFalse(hash(a) == hash(d))

    class GameStateTest(unittest.TestCase):
        def test_move_to_top_right(self):
            result = [
                GameState(
                    [
                        [
                            Card(RED, 9),
                            Card(GREEN, 9),
                            Card(BLACK, 9),
                            Card(SPECIAL, 1),
                        ],
                        [],
                        [],
                        [],
                        [],
                        [],
                        [],
                        [],
                    ],
                    [Card(FACE_DOWN, None)] * 3,
                    [0, 8, 8, 8],
                )
            ]
            for i in range(4):
                state = copy.deepcopy(result[-1])
                self.assertFalse(state.is_solved())
                self.assertTrue(state.can_move_to_top_right_storage())
                state.move_to_top_right_storage()
                result.append(state)

            self.assertTrue(state.is_solved())

        def test_move_storage_to_top_right(self):
            result = [
                GameState(
                    [
                        [Card(RED, 8)],
                        [Card(RED, None)],
                        [Card(RED, None)],
                        [Card(RED, None)],
                        [Card(RED, None)],
                        [],
                        [],
                        [],
                    ],
                    [
                        Card(RED, 9),
                        Card(FACE_DOWN, None),
                        Card(FACE_DOWN, None),
                    ],
                    [1, 7, 9, 9],
                )
            ]
            for i in range(2):
                state = copy.deepcopy(result[-1])
                self.assertFalse(state.is_solved())
                self.assertTrue(state.can_move_to_top_right_storage())
                state.move_to_top_right_storage()
                result.append(state)

            for s in result:
                print(s)

        def test_hashable(self):
            empty_columns = [[] for _ in range(8)]
            a = copy.deepcopy(empty_columns)
            a[0].append(Card(RED, 1))

            b = copy.deepcopy(empty_columns)
            b[1].append(Card(RED, 1))

            c = copy.deepcopy(b)

            state_a = GameState(a)
            state_b = GameState(b)
            state_c = GameState(c)

            self.assertNotEqual(state_a, state_b)
            self.assertNotEqual(hash(state_a), hash(state_b))

            self.assertEqual(state_b, state_c)
            self.assertEqual(hash(state_b), hash(state_c))

        def test_can_move_top_left_to_column(self):
            state = GameState(
                [
                    [Card(RED, None), Card(RED, None), Card(RED, None)],
                    [Card(RED, 9)],
                    [Card(GREEN, 9)],
                    [],
                    [],
                    [],
                    [],
                    [],
                ],
                [Card(FACE_DOWN, None), Card(GREEN, None), Card(RED, 8)],
                [9, 8, 9, 9],
            )

            # face down can't move even to a free column
            self.assertFalse(state.can_move_top_left_to_column(0, 7))

            # dragon can move only to a free column
            self.assertFalse(state.can_move_top_left_to_column(1, 0))
            self.assertFalse(state.can_move_top_left_to_column(1, 1))
            self.assertFalse(state.can_move_top_left_to_column(1, 2))
            self.assertTrue(state.can_move_top_left_to_column(1, 7))

            # regular card can move onto another card of a different suit and
            # one lower value
            self.assertFalse(state.can_move_top_left_to_column(2, 0))
            self.assertFalse(state.can_move_top_left_to_column(2, 1))
            self.assertTrue(state.can_move_top_left_to_column(2, 2))
            self.assertTrue(state.can_move_top_left_to_column(2, 7))

            state.move_top_left_to_column(2, 2)
            self.assertEqual(2, len(state.top_left_storage))
            self.assertEqual(2, len(state.columns[2]))
            self.assertEqual(Card(RED, 8), state.columns[2][-1])

    class SolitaireTest(unittest.TestCase):
        def setUp(self):
            self.solved = GameState(
                [[], [], [], [], [], [], [], []],
                [Card(FACE_DOWN, None)] * 3,
                [1, 9, 9, 9],
            )
            self.almost_solved = GameState(
                [[Card(RED, 9)], [], [], [], [], [], [], []],
                [Card(FACE_DOWN, None)] * 3,
                [1, 8, 9, 9],
            )

        def test_is_solved(self):
            self.assertTrue(self.solved.is_solved())

            # The base case
            game = Game()
            result = game.play(self.solved)
            self.assertEqual(1, len(result))
            self.assertEqual(self.solved, result[0])

        def test_move_to_top_right_solve(self):
            self.assertFalse(self.almost_solved.is_solved())

            # Solved after a single iteration
            game = Game()
            result = game.play(self.almost_solved)
            self.assertEqual(2, len(result))
            self.assertEqual(self.almost_solved, result[0])
            self.assertTrue(result[-1].is_solved())

    debug_print()

    print("tests")
    unittest.main()
