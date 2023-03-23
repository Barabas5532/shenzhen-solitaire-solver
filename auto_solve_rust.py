import shutil
import uuid
from dataclasses import dataclass, replace

import cv2  # type: ignore
import pyautogui  # type: ignore
import shenzhen_solitaire_solver_rust  # type: ignore

import solitaire
from image_processing.card_position_processing import get_state_from_image
from image_processing.image_list import CardImageNames


def solve_game() -> None:
    new_game()
    create_screenshot()
    solution = solve_screenshot()
    if solution is None:
        print("Failed to find solution")
        filename = f"bug-{uuid.uuid1()}.png"
        shutil.copy("game.png", filename)
        return
    play_game(solution)


def debug_print() -> None:
    width, height = pyautogui.size()

    print(f"pyautogui detected screen size {width}x{height}")


def new_game() -> None:
    # TODO get pyautogui to click the new game button, then wait long enough
    #      for animations to complete
    print("Creating a new game...")
    move_mouse(MouseActionClick(1500, 930))
    pyautogui.sleep(5)


def create_screenshot() -> None:
    screenshot = pyautogui.screenshot(region=(400, 380, 1200, 380))
    screenshot.save("game.png")


def solve_screenshot() -> (
    None | list[tuple[solitaire.GameState, solitaire.GameMoveBase]]
):
    state = get_state_from_image(cv2.imread("game.png"))
    print(f"Found initial state:\n{state}")

    solution = shenzhen_solitaire_solver_rust.solve_game(state.to_json())
    print(solution)
    return solution


def play_game(
    solution: list[tuple[solitaire.GameState, solitaire.GameMoveBase]]
) -> None:
    for step in solution:
        print(f"Current game state is:\n{step[0]}")
        print(f"The next move is: {step[1]}")
        make_move(step)


class ScreenCoordinates:
    COLUMN_START_COORDINATES = (450, 400)
    COLUMN_HORIZONTAL_SPACE = 152
    COLUMN_VERTICAL_SPACE = 31

    FOUNDATION_BOUNDS = (1150, 110, 460, 260)
    FOUNDATION_IMAGES = [
        "./image_processing/images/FOUNDATION_0.png",
        "./image_processing/images/FOUNDATION_1.png",
        "./image_processing/images/FOUNDATION_2.png",
    ]

    CELL_BOUNDS = (380, 110, 460, 260)
    CELL_IMAGES = [
        "./image_processing/images/CELL_0.png",
        "./image_processing/images/CELL_1.png",
        "./image_processing/images/CELL_2.png",
    ]

    def get_column(self, index: int, depth: int) -> tuple[int, int]:
        return (
            self.COLUMN_START_COORDINATES[0]
            + index * self.COLUMN_HORIZONTAL_SPACE,
            self.COLUMN_START_COORDINATES[1]
            + depth * self.COLUMN_VERTICAL_SPACE,
        )

    def get_foundation(self, card: solitaire.Card) -> tuple[int, int]:
        if card.suit == solitaire.Suit.SPECIAL:
            return (1030, 240)

        if card.value == 1:
            # We are looking for an empty foundation
            for i, image in enumerate(self.FOUNDATION_IMAGES):
                res = pyautogui.locateOnScreen(
                    image, region=self.FOUNDATION_BOUNDS
                )
                if res:
                    return tuple(pyautogui.center(res))

            raise ValueError("Can't find empty foundation")
        else:
            # We are looking for the card with a value one below the current
            # card
            assert card.value is not None
            search_card = replace(card, value=card.value - 1)
            image = card_image_names.get_name(search_card)
            res = pyautogui.locateOnScreen(
                image, region=self.FOUNDATION_BOUNDS
            )
            if res:
                return tuple(pyautogui.center(res))
            raise ValueError(f"Can't find card {search_card} in foundation")

    def get_cell(self, card: None | solitaire.Card = None) -> tuple[int, int]:
        if card is None:
            # We are looking for an empty cell
            for i, image in enumerate(self.CELL_IMAGES):
                res = pyautogui.locateOnScreen(image, region=self.CELL_BOUNDS)
                if res:
                    return tuple(pyautogui.center(res))

            raise ValueError("Can't find empty cell")
        else:
            # We are looking for the card to move
            image = card_image_names.get_name(card)
            res = pyautogui.locateOnScreen(image, region=self.CELL_BOUNDS)
            if res:
                return tuple(pyautogui.center(res))

            raise ValueError(f"Can't find card {card} in cells")


@dataclass(frozen=True)
class MouseAction:
    pass


@dataclass(frozen=True)
class MouseActionClick(MouseAction):
    x: int
    y: int


@dataclass(frozen=True)
class MouseActionDrag(MouseAction):
    x_start: int
    y_start: int
    x_end: int
    y_end: int


def get_move(
    step: tuple[solitaire.GameState, solitaire.GameMoveBase]
) -> MouseAction:
    state = step[0]
    move = step[1]

    if move is None:
        raise ValueError("Move is None")

    if isinstance(move, solitaire.GameMoveCollectDragons):
        print(f"collecting {move.suit} dragons")
        y_coordinate = {
            solitaire.Suit.RED: 160,
            solitaire.Suit.GREEN: 240,
            solitaire.Suit.BLACK: 320,
        }
        return MouseActionClick(890, y_coordinate[move.suit])

    source = (0, 0)
    destination = (0, 0)

    if isinstance(move, solitaire.GameMoveColumnToTopRightStorage):
        print(f"move column {move.column} to foundation")
        source = screen_coordinates.get_column(
            move.column, len(state.columns[move.column]) - 1
        )
        # The game might have already moved this card here automatically.
        # We don't know the exact logic the game uses when doing this.
        # Therefore, we should check the actual layout every time we need to
        # make this move. If we have seen a suit in a partical slot, that will
        # never change.

        # If the card has already been moved to the storage area by the game,
        # we could skip the move?
        destination = screen_coordinates.get_foundation(
            state.columns[move.column][-1]
        )

    if isinstance(move, solitaire.GameMoveTopLeftToTopRightStorage):
        card = state.top_left_storage[move.top_left_index]
        print(f"move {card} from cell to foundation")
        source = screen_coordinates.get_cell(card)
        destination = screen_coordinates.get_foundation(card)

    if isinstance(move, solitaire.GameMoveColumnToOtherColumn):
        print(
            f"move {move.stack_size} cards from column "
            f"{move.from_column_index} to column {move.to_column_index}"
        )
        source = screen_coordinates.get_column(
            move.from_column_index,
            len(state.columns[move.from_column_index]) - move.stack_size,
        )
        destination = screen_coordinates.get_column(
            move.to_column_index, len(state.columns[move.to_column_index]) - 1
        )
        pass

    if isinstance(move, solitaire.GameMoveToTopLeftStorage):
        card = state.columns[move.column][-1]
        print(f"move {card} to cell")
        source = screen_coordinates.get_column(
            move.column, len(state.columns[move.column]) - 1
        )
        destination = screen_coordinates.get_cell()

    if isinstance(move, solitaire.GameMoveTopLeftToColumn):
        card = state.top_left_storage[move.top_left_index]
        print(f"move {card} from cell to column {move.column_index}")
        source = screen_coordinates.get_cell(card)
        destination = screen_coordinates.get_column(
            move.column_index, len(state.columns[move.column_index]) - 1
        )

    return MouseActionDrag(*source, *destination)


def move_mouse(move: MouseAction) -> None:
    if isinstance(move, MouseActionClick):
        print(f"Clicking at {(move.x, move.y)}")
        pyautogui.moveTo(1, 1)
        pyautogui.moveTo(move.x, move.y)
        # click function seems to be too fast, and doesn't get detected by the
        # game
        pyautogui.mouseDown()
        pyautogui.mouseUp()
    elif isinstance(move, MouseActionDrag):
        print(
            f"dragging mouse from {(move.x_start, move.y_start)} to "
            f"{(move.x_end, move.y_end)}"
        )
        pyautogui.moveTo(1, 1)
        pyautogui.moveTo(move.x_start, move.y_start)
        pyautogui.mouseDown()
        pyautogui.moveTo(move.x_end, move.y_end, 0.25)
        pyautogui.mouseUp()
        pass

    # move the mouse out of the way
    pyautogui.moveTo(1, 1)


def make_move(
    step: tuple[solitaire.GameState, solitaire.GameMoveBase]
) -> None:
    try:
        move = get_move(step)
    except ValueError as e:
        print(f"Skipping move because of expection: {e}")
        return
    move_mouse(move)
    pyautogui.sleep(0.5)


number_of_solves = 50
screen_coordinates = ScreenCoordinates()
card_image_names = CardImageNames("./image_processing/images")

debug_print()

# time.sleep(5)
# move_mouse(MouseActionClick(1270, 940))

# start = screen_coordinates.get_column(0, 4)
# end = screen_coordinates.get_column(4, 3)
# move_mouse(MouseActionDrag(*start, *end))

# print(screen_coordinates.get_foundation(solitaire.Card(solitaire.Suit.GREEN, 2)))
# print(screen_coordinates.get_foundation(solitaire.Card(solitaire.Suit.RED, 1)))
# print(screen_coordinates.get_cell(solitaire.Card(solitaire.Suit.RED, None)))
# print(screen_coordinates.get_cell(solitaire.Card(solitaire.Suit.BLACK, None)))
# print(screen_coordinates.get_cell())

for i in range(number_of_solves):
    solve_game()
