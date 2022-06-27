import itertools

RED = 0
GREEN = 1
BLACK = 2
SPECIAL = 3

class Card:
    colors = ["🟥", "🟩", "⬛", "🆒"]

    def __init__(self, suit, value: int):
        self.suit = suit
        self.value = value

    def __str__(self):
        if self.suit is None: return "xxx"
        return f"{self.colors[self.suit]}{self.value if self.value is not None else 'x'}";

    # quick hack to make printing an array of card readable
    def __repr__(self):
        return self.__str__()

class GameState:

    # TODO top left makes more sense as a set, the order of the cards and
    # spaces does not matter, same for the top right storage
    def __init__(self, centre, top_left_storage=set(), top_right_storage=set()):
        # scratch pad to temporarty store cards
        # a space is lost when dragons are stacked here, represented by a
        # Card(None, None)
        self.top_left_storage = top_left_storage
        # The aim of the game is to get all the cards stacked here
        self.top_right_storage = top_right_storage

        # The main play area, where all of the cards are placed at the start
        self.centre = centre

    # All the columns in the centre have no cards
    def is_winning():
        for column in centre:
            if column.size != 0:
                return False

        return True

    def __str__(self):
        top_row = "========== GAME STATE =========\n"
        for i, card in enumerate(self.top_left_storage):
            top_row += str(card)
            top_row += " "

        # TODO show the special card here if it's placed, and get rid of +1
        for i in range(3 - len(self.top_left_storage) + 2):
            top_row += "    "

        for i, card in enumerate(self.top_right_storage):
            top_row += str(card)
            top_row += " "

        # transpopse rows and columns so we can print the cards in the layout
        # that matches the game
        transposed = list(map(list, itertools.zip_longest(*self.centre, fillvalue=None)))

        centre = ""
        for i, column in enumerate(transposed):
            for j, card in enumerate(column):
                if card is None:
                    centre += "   "
                else:
                    centre += str(card)
                centre += ' '
            centre += "\n"

        return top_row + "\n" + centre


class Game:
    def __init__():
        self.seen_states = set()

    def already_seen(self, state):
        if state in self.seen_state:
            return True

        self.seen_states.add(state)
        return False

    def play(state: GameState):
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
        if already_seen(state):
            return None

        # TODO need to keep a record of moves or states from the start of the
        # game, and show these in order after a solution is found. maybe we
        # should return prepend our state to the returned state in the
        # recursive call

        if state.is_winning():
            return [state]

        # The input state is owned by the caller, and we must not modify it
        state_copy = state.deepcopy()

        if state_copy.can_move_to_top_right_storage():
            copy.move_to_top_right_storage()
            result = play(copy)
            return [state, *result] if result is not None else None

        return None

if __name__ == "__main__":
    print(f"|{Card(suit=RED, value=5)}|")
    print(f"|{Card(suit=BLACK, value=0)}|")
    print(f"|{Card(suit=GREEN, value=None)}|")
    print(f"|{Card(suit=SPECIAL, value=None)}|")


    # From https://shenzhen-io.fandom.com/wiki/Shenzhen_Solitaire
    # https://shenzhen-io.fandom.com/wiki/File:Solitaire.png
    column0 = [Card(BLACK, None),
               Card(RED, None),
               Card(BLACK, 7),
               Card(RED, 7),
               Card(BLACK, 6),]

    column1 = [Card(RED, None),
               Card(RED, 9),
               Card(GREEN, 8),
               Card(GREEN, 7),
               Card(RED, 4),]

    column2 = [Card(GREEN, 2),
               Card(BLACK, 3),
               Card(BLACK, 5),
               Card(RED, 5),
               Card(GREEN, 3),]

    column3 = [Card(GREEN, 1),
               Card(RED, None),
               Card(SPECIAL, None),
               Card(RED, 1),
               Card(GREEN, 6),]

    column4 = [Card(GREEN, 4),
               Card(RED, 8),
               Card(RED, 2),
               Card(RED, 6),
               Card(BLACK, None),]

    column5 = [Card(GREEN, 5),
               Card(BLACK, 4),
               Card(RED, None),
               Card(BLACK, 1),
               Card(BLACK, 8),]

    column6 = [Card(GREEN, None),
               Card(RED, 3),
               Card(GREEN, None),
               Card(BLACK, 2),
               Card(GREEN, None),]

    column7 = [Card(BLACK, 9),
               Card(BLACK, None),
               Card(GREEN, None),
               Card(GREEN, 9),
               Card(BLACK, None),]

    centre = [
            column0,
            column1,
            column2,
            column3,
            column4,
            column5,
            column6,
            column7,
            ]

    state = GameState(centre, {Card(None, None), Card(BLACK, 3)}, [Card(RED, 1), Card(GREEN, 2), Card(BLACK, 3)])
    print(state)

    progressed_centre = [
            [],
            column1[:2],
            column2,
            column3[:2],
            column4[:1],
            [],
            column6,
            column7[:2],
            ]
    progressed_state = GameState(progressed_centre)
    print(progressed_state)
