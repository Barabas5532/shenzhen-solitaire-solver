import itertools
import copy
from typing import Optional, List, Tuple, Set
from enum import IntEnum

Columns = Tuple[
    List["Card"],
    List["Card"],
    List["Card"],
    List["Card"],
    List["Card"],
    List["Card"],
    List["Card"],
    List["Card"],
]


class Suit(IntEnum):
    SPECIAL = 0
    RED = 1
    GREEN = 2
    BLACK = 3
    FACE_DOWN = 4


class CardLocation(IntEnum):
    TOP_LEFT = 0
    CENTRE = 1


class Card:
    colors = ["🆒", "🟥", "🟩", "⬛"]

    def __init__(self, suit: int, value: Optional[int]) -> None:
        self.suit: int = suit
        self.value: Optional[int] = value

    def __str__(self) -> str:
        if self.suit is Suit.FACE_DOWN:
            return "xxx"
        return (
            f"{self.colors[self.suit]}"
            f"{self.value if self.value is not None else 'x'}"
        )

    # quick hack to make printing an array of card readable
    def __repr__(self) -> str:
        return self.__str__()

    def __hash__(self) -> int:
        return hash(frozenset((self.suit, self.value)))

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, type(self)):
            assert False
            return False

        return (self.suit, self.value) == (other.suit, other.value)

    def is_dragon(self, suit: Optional[Suit] = None) -> bool:
        if suit is None:
            return self.value is None

        return self.suit == suit and self.value is None

    def can_be_moved_on_top_of(self, other: "Card") -> bool:
        assert self.suit in [Suit.BLACK, Suit.GREEN, Suit.RED]

        # can't move on top of dragon
        if other.is_dragon():
            return False

        # dragon can't go on top of any other card
        if self.is_dragon():
            return False

        assert other.value is not None
        return self.suit != other.suit and self.value == other.value - 1


class CardPosition:
    def __init__(self, location: CardLocation, index: int):
        self.location = location
        self.index = index


class GameState:
    # TODO use tuple to strongly type a fixed length sequence for columns
    def __init__(
        self,
        columns: Columns,
        top_left_storage: List = [],
        top_right_storage: List = [0 for _ in range(4)],
    ) -> None:
        assert len(top_left_storage) <= 3

        # scratch pad to temporarily store cards
        # a space is lost when dragons are stacked here, represented by a
        # Card(Suit.FACE_DOWN, None)
        self.top_left_storage = top_left_storage
        # The aim of the game is to get all the cards stacked here
        self.top_right_storage = top_right_storage

        # The main play area, where all of the cards are placed at the start
        self.columns = columns

    # All the columns in the centre have no cards
    def is_solved(self) -> bool:
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
            assert self.top_left_storage[i].suit == Suit.FACE_DOWN

        return True

    # TODO Suit.SPECIAL card can always be moved to storage, it's hardcoded to
    # have value of 1 for now
    def _get_move_to_top_right_storage(self) -> Optional[CardPosition]:
        """Get the position of a card that can be moved to the top right
        storage, if any"""
        for i, column in enumerate(self.columns):
            if len(column) == 0:
                continue
            card = column[-1]
            if (
                card.value == 1
                or card.value is not None
                and self.top_right_storage[card.suit] == card.value - 1
            ):
                return CardPosition(CardLocation.CENTRE, i)

        for i, card in enumerate(self.top_left_storage):
            assert card.value != 1
            if (
                card.value is not None
                and self.top_right_storage[card.suit] == card.value - 1
            ):
                return CardPosition(CardLocation.TOP_LEFT, i)

        return None

    def can_move_to_top_right_storage(self) -> bool:
        return self._get_move_to_top_right_storage() is not None

    def move_to_top_right_storage(self) -> None:
        card_position = self._get_move_to_top_right_storage()
        assert card_position is not None
        column_to_move = card_position.index

        if card_position.location == CardLocation.CENTRE:
            card = self.columns[column_to_move].pop()
        elif card_position.location == CardLocation.TOP_LEFT:
            card = self.top_left_storage.pop(card_position.index)
        else:
            assert False

        self.top_right_storage[card.suit] = card.value

    def can_move_top_left_to_column(
        self,
        top_left_index: int,
        column_index: int,
    ) -> bool:
        if top_left_index >= len(self.top_left_storage):
            return False

        card_to_move = self.top_left_storage[top_left_index]
        # Can't move collected dragons
        if card_to_move.suit is Suit.FACE_DOWN:
            return False

        # We are sure that the top left storage has a movable card now

        # moving to an empty column is always allowed
        if len(self.columns[column_index]) == 0:
            return True

        target_card = self.columns[column_index][-1]
        if target_card.is_dragon():
            return False

        return card_to_move.can_be_moved_on_top_of(target_card)

    def move_top_left_to_column(
        self, top_left_index: int, column_index: int
    ) -> None:
        self.columns[column_index].append(
            self.top_left_storage.pop(top_left_index)
        )

    def can_move_column_to_top_left(self, column_index: int) -> bool:
        return (
            len(self.columns[column_index]) != 0
            and len(self.top_left_storage) < 3
        )

    def move_column_to_top_left(self, column_index: int) -> None:
        self.top_left_storage.append(self.columns[column_index].pop())
        assert len(self.top_left_storage) <= 3

    def can_collect_dragons(self, suit: Suit) -> bool:
        if 3 == len(
            list(
                filter(
                    lambda card: not (card.is_dragon(suit)),
                    self.top_left_storage,
                )
            )
        ):
            return False

        free_dragon_count = 0
        for column in self.columns:
            for card in reversed(column):
                if card.is_dragon(suit):
                    free_dragon_count += 1
                else:
                    # check next column
                    break

        for card in self.top_left_storage:
            if card.is_dragon(suit):
                free_dragon_count += 1

        return free_dragon_count == 4

    def collect_dragons(self, suit: Suit) -> None:
        # This is always called after checking if this move is valid.
        # Therefore, we can just remove all the dragons and add a face down
        # card to the top left.
        for column in self.columns:
            column[:] = [card for card in column if not card.is_dragon(suit)]

        self.top_left_storage = [
            card for card in self.top_left_storage if not card.is_dragon(suit)
        ]
        self.top_left_storage.append(Card(Suit.FACE_DOWN, None))
        assert len(self.top_left_storage) <= 3

    def _get_column_stack_size(self, column_index: int) -> int:
        if 0 == len(self.columns[column_index]):
            return 0

        stack_size = 1

        column = self.columns[column_index]
        for i, card in enumerate(column):
            if i + 1 == len(column):
                break

            next_card = column[i + 1]

            if next_card.can_be_moved_on_top_of(card):
                stack_size += 1
            else:
                stack_size = 1

        return stack_size

    def can_move_column_to_other_column(
        self,
        *,
        from_column_index: int,
        to_column_index: int,
        stack_size: int,
    ) -> bool:
        actual_stack_size = self._get_column_stack_size(from_column_index)

        # TODO this statement is redundant, stack size is always greater than
        #      zero. Remove once we have enough test coverage
        if actual_stack_size == 0:
            return False

        if stack_size > actual_stack_size:
            return False

        if len(self.columns[to_column_index]) == 0:
            return True

        stack_first_card = self.columns[from_column_index][-stack_size]
        target_card = self.columns[to_column_index][-1]
        return stack_first_card.can_be_moved_on_top_of(target_card)

    def move_column_to_other_column(
        self,
        *,
        from_column_index: int,
        to_column_index: int,
        stack_size: int,
    ) -> None:
        from_column = self.columns[from_column_index]
        to_column = self.columns[to_column_index]

        card_stack = from_column[-stack_size:]
        del from_column[-stack_size:]
        to_column.extend(card_stack)

    def __str__(self) -> str:
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
            map(
                list,  # type: ignore
                itertools.zip_longest(*self.columns, fillvalue=None),
            )
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

    def _tuple(self) -> Tuple:
        return (
            tuple(self.top_left_storage),
            tuple([tuple(column) for column in self.columns]),
            tuple(self.top_right_storage),
        )

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, type(self)):
            assert False
            return False

        return self._tuple() == other._tuple()

    def __hash__(self) -> int:
        return hash(self._tuple())


class Game:
    def __init__(self) -> None:
        self.seen_states: Set[GameState] = set()

    def already_seen(self, state: GameState) -> bool:
        if state in self.seen_states:
            return True

        self.seen_states.add(state)
        return False

    def play(self, state: GameState) -> Optional[List[GameState]]:
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

            # TODO there might be some edge cases where the game doesn't force
            # you to make this move. Those could be the states where this move
            # actually makes you lose the game.
            #
            # E.G. when moving the card would mean a free columns stays blocked
            # by another card.
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
        for column_index in range(8):
            if state_copy.can_move_column_to_top_left(column_index):
                state_copy.move_column_to_top_left(column_index)
                result = self.play(state_copy)
                if result is not None:
                    return [state, *result]
                # else we keep looping to try all the possible moves

        # collect dragons
        for suit in [Suit.RED, Suit.GREEN, Suit.BLACK]:
            if state_copy.can_collect_dragons(suit):
                state_copy.collect_dragons(suit)
                result = self.play(state_copy)
                if result is not None:
                    return [state, *result]

        # move any set of cards from any column to any other column

        return None


if __name__ == "__main__":

    def debug_print() -> None:
        print(Card(suit=Suit.RED, value=5))
        print(Card(suit=Suit.BLACK, value=0))
        print(Card(suit=Suit.GREEN, value=None))
        print(Card(suit=Suit.SPECIAL, value=1))

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

        state = GameState(columns)
        print(state)

        # XXX: This is an invalid game state, for printing demo only
        progressed_columns: Columns = (
            [],
            column1[:2],
            column2,
            column3[:2],
            column4[:1],
            [],
            column6,
            column7[:2],
        )
        progressed_state = GameState(
            progressed_columns, [Card(Suit.FACE_DOWN, None)], [1, 2, 3, 4]
        )
        print(progressed_state)

    import unittest

    class CardTest(unittest.TestCase):
        def test_hashable(self) -> None:
            a = Card(Suit.RED, 1)
            b = Card(Suit.RED, 1)
            c = Card(Suit.GREEN, 1)
            d = Card(Suit.RED, 2)

            self.assertFalse(a is b)
            self.assertTrue(a == b)
            self.assertTrue(hash(a) == hash(b))

            self.assertFalse(a == c)
            self.assertFalse(hash(a) == hash(c))

            self.assertFalse(a == d)
            self.assertFalse(hash(a) == hash(d))

    class GameStateTest(unittest.TestCase):
        def test_move_to_top_right(self) -> None:
            result = [
                GameState(
                    (
                        [
                            Card(Suit.RED, 9),
                            Card(Suit.GREEN, 9),
                            Card(Suit.BLACK, 9),
                            Card(Suit.SPECIAL, 1),
                        ],
                        [],
                        [],
                        [],
                        [],
                        [],
                        [],
                        [],
                    ),
                    [Card(Suit.FACE_DOWN, None)] * 3,
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

        def test_move_storage_to_top_right(self) -> None:
            result = [
                GameState(
                    (
                        [Card(Suit.RED, 8)],
                        [Card(Suit.RED, None)],
                        [Card(Suit.RED, None)],
                        [Card(Suit.RED, None)],
                        [Card(Suit.RED, None)],
                        [],
                        [],
                        [],
                    ),
                    [
                        Card(Suit.RED, 9),
                        Card(Suit.FACE_DOWN, None),
                        Card(Suit.FACE_DOWN, None),
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

        def test_hashable(self) -> None:
            empty_columns: Columns = ([], [], [], [], [], [], [], [])
            a = copy.deepcopy(empty_columns)
            a[0].append(Card(Suit.RED, 1))

            b = copy.deepcopy(empty_columns)
            b[1].append(Card(Suit.RED, 1))

            c = copy.deepcopy(b)

            state_a = GameState(a)
            state_b = GameState(b)
            state_c = GameState(c)

            self.assertNotEqual(state_a, state_b)
            self.assertNotEqual(hash(state_a), hash(state_b))

            self.assertEqual(state_b, state_c)
            self.assertEqual(hash(state_b), hash(state_c))

        def test_can_move_top_left_to_column(self) -> None:
            state = GameState(
                (
                    [
                        Card(Suit.RED, None),
                        Card(Suit.RED, None),
                        Card(Suit.RED, None),
                    ],
                    [Card(Suit.RED, 9)],
                    [Card(Suit.GREEN, 9)],
                    [],
                    [],
                    [],
                    [],
                    [],
                ),
                [
                    Card(Suit.FACE_DOWN, None),
                    Card(Suit.GREEN, None),
                    Card(Suit.RED, 8),
                ],
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
            self.assertEqual(Card(Suit.RED, 8), state.columns[2][-1])

        def test_can_move_column_to_top_left(self) -> None:
            state = GameState(
                (
                    [
                        Card(Suit.RED, 9),
                    ],
                    [
                        Card(Suit.RED, 8),
                    ],
                    [
                        Card(Suit.RED, 7),
                    ],
                    [
                        Card(Suit.RED, None),
                        Card(Suit.RED, None),
                        Card(Suit.RED, None),
                    ],
                    [],
                    [],
                    [],
                    [],
                ),
                [
                    Card(Suit.FACE_DOWN, None),
                    Card(Suit.FACE_DOWN, None),
                ],
                [9, 6, 9, 9],
            )

            # Given a single empty slot in the top left
            self.assertEqual(2, len(state.top_left_storage))

            # Can't move if there is not any cards in the column
            self.assertFalse(state.can_move_column_to_top_left(7))

            # Can move if there is any card in the column
            self.assertTrue(state.can_move_column_to_top_left(0))
            self.assertTrue(state.can_move_column_to_top_left(1))
            self.assertTrue(state.can_move_column_to_top_left(2))
            self.assertTrue(state.can_move_column_to_top_left(3))

            # Moving a card causes it to disappear from the column
            moved_card = state.columns[0][-1]
            state.move_column_to_top_left(0)
            self.assertEqual(0, len(state.columns[0]))
            # The top left storage should be filled up
            self.assertEqual(3, len(state.top_left_storage))
            # The moved card should appear in the top left
            self.assertIn(moved_card, state.top_left_storage)

            # Now that the top left is filled up, no cards can be moved there
            self.assertFalse(state.can_move_column_to_top_left(1))

        def test_collect_dragons(self) -> None:
            state = GameState(
                (
                    [
                        Card(Suit.GREEN, None),
                    ],
                    [
                        Card(Suit.GREEN, None),
                    ],
                    [
                        Card(Suit.GREEN, None),
                    ],
                    [
                        Card(Suit.GREEN, None),
                    ],
                    [
                        Card(Suit.RED, None),
                        Card(Suit.RED, None),
                        Card(Suit.RED, None),
                    ],
                    [
                        Card(Suit.BLACK, None),
                        Card(Suit.BLACK, None),
                    ],
                    [
                        Card(Suit.BLACK, None),
                        Card(Suit.RED, 9),
                    ],
                    [],
                ),
                [
                    Card(Suit.RED, None),
                ],
                [1, 8, 9, 9],
            )

            self.assertTrue(state.can_collect_dragons(Suit.GREEN))
            self.assertTrue(state.can_collect_dragons(Suit.RED))
            self.assertFalse(state.can_collect_dragons(Suit.BLACK))

            state.collect_dragons(Suit.GREEN)
            self.assertEqual(0, len(state.columns[0]))
            self.assertEqual(0, len(state.columns[1]))
            self.assertEqual(0, len(state.columns[2]))
            self.assertEqual(0, len(state.columns[3]))
            self.assertIn(Card(Suit.FACE_DOWN, None), state.top_left_storage)
            self.assertIn(Card(Suit.RED, None), state.top_left_storage)
            self.assertEqual(2, len(state.top_left_storage))

            state.collect_dragons(Suit.RED)
            self.assertEqual(0, len(state.columns[4]))
            self.assertEqual(2, len(state.top_left_storage))

        def test_can_not_collect_dragons_when_storage_full(self) -> None:
            state = GameState(
                (
                    [
                        Card(Suit.RED, None),
                    ],
                    [
                        Card(Suit.RED, None),
                    ],
                    [
                        Card(Suit.RED, None),
                    ],
                    [
                        Card(Suit.RED, None),
                    ],
                    [],
                    [
                        Card(Suit.BLACK, None),
                        Card(Suit.BLACK, None),
                    ],
                    [
                        Card(Suit.BLACK, None),
                    ],
                    [],
                ),
                [
                    Card(Suit.FACE_DOWN, None),
                    Card(Suit.RED, 9),
                    Card(Suit.BLACK, None),
                ],
                [1, 8, 9, 9],
            )

            # We can't collect dragond if the top left storage is full
            self.assertFalse(state.can_collect_dragons(Suit.RED))
            # Except if the blocking card is a dragon that we wanted to collect
            self.assertTrue(state.can_collect_dragons(Suit.BLACK))

            state.collect_dragons(Suit.BLACK)
            self.assertEqual(3, len(state.top_left_storage))
            for column in state.columns:
                self.assertNotIn(Card(Suit.BLACK, None), column)

        def test_move_column_to_other_column(self) -> None:
            state = GameState(
                (
                    [
                        Card(Suit.RED, 9),
                    ],
                    [
                        Card(Suit.GREEN, 8),
                        Card(Suit.BLACK, 7),
                    ],
                    [
                        Card(Suit.RED, 8),
                    ],
                    [
                        Card(Suit.BLACK, 9),
                        Card(Suit.GREEN, 9),
                        Card(Suit.BLACK, 8),
                    ],
                    [],
                    [],
                    [],
                    [],
                ),
                [
                    Card(Suit.FACE_DOWN, None),
                    Card(Suit.FACE_DOWN, None),
                    Card(Suit.FACE_DOWN, None),
                ],
                [1, 7, 7, 6],
            )

            # Can't move if the source column is empty
            self.assertFalse(
                state.can_move_column_to_other_column(
                    from_column_index=4,
                    to_column_index=7,
                    stack_size=1,
                )
            )

            # Can't move if the source column stack size is less than the
            # requested stack size
            self.assertFalse(
                state.can_move_column_to_other_column(
                    from_column_index=0,
                    to_column_index=7,
                    stack_size=2,
                )
            )

            # Can move to an empty column
            self.assertTrue(
                state.can_move_column_to_other_column(
                    from_column_index=0,
                    to_column_index=7,
                    stack_size=1,
                )
            )
            state_copy = copy.deepcopy(state)
            card_to_move = state_copy.columns[0][-1]
            state_copy.move_column_to_other_column(
                from_column_index=0,
                to_column_index=7,
                stack_size=1,
            )
            self.assertEqual(0, len(state_copy.columns[0]))
            self.assertNotIn(card_to_move, state_copy.columns[0])
            self.assertEqual(1, len(state_copy.columns[7]))
            self.assertIn(card_to_move, state_copy.columns[7])

            # Can move stack to empty column
            self.assertTrue(
                state.can_move_column_to_other_column(
                    from_column_index=3,
                    to_column_index=7,
                    stack_size=2,
                )
            )

            # Can move stack on top of another card
            self.assertTrue(
                state.can_move_column_to_other_column(
                    from_column_index=1,
                    to_column_index=0,
                    stack_size=2,
                )
            )
            state_copy = copy.deepcopy(state)
            cards_to_move = state_copy.columns[1][-2:]
            state_copy.move_column_to_other_column(
                from_column_index=1,
                to_column_index=0,
                stack_size=2,
            )
            self.assertEqual(0, len(state_copy.columns[1]))
            for card_to_move in cards_to_move:
                self.assertNotIn(card_to_move, state_copy.columns[1])
            self.assertEqual(3, len(state_copy.columns[0]))
            for card_to_move in cards_to_move:
                self.assertIn(card_to_move, state_copy.columns[0])

    class SolitaireTest(unittest.TestCase):
        def setUp(self) -> None:
            self.solved = GameState(
                ([], [], [], [], [], [], [], []),
                [Card(Suit.FACE_DOWN, None)] * 3,
                [1, 9, 9, 9],
            )
            self.almost_solved = GameState(
                ([Card(Suit.RED, 9)], [], [], [], [], [], [], []),
                [Card(Suit.FACE_DOWN, None)] * 3,
                [1, 8, 9, 9],
            )

        def test_is_solved(self) -> None:
            self.assertTrue(self.solved.is_solved())

            # The base case
            game = Game()
            result = game.play(self.solved)
            self.assertIsNotNone(result)
            assert result is not None  # for mypy type narrowing
            self.assertEqual(1, len(result))
            self.assertEqual(self.solved, result[0])

        def test_move_to_top_right_solve(self) -> None:
            self.assertFalse(self.almost_solved.is_solved())

            # Solved after a single iteration
            game = Game()
            result = game.play(self.almost_solved)
            self.assertIsNotNone(result)
            assert result is not None  # for mypy type narrowing
            self.assertEqual(2, len(result))
            self.assertEqual(self.almost_solved, result[0])
            self.assertTrue(result[-1].is_solved())

    debug_print()

    print("tests")
    unittest.main()
