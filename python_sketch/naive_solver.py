from typing import Iterable, Dict
from impartial_game import ImpartialGame


def mex(numbers: Iterable[int]) -> int:
    """Compute the minimum excluded value (mex) for a sequence of non-negative integers."""
    numbers_set = set(numbers)
    i = 0
    while i in numbers_set:
        i += 1
    return i


class NaiveSolver:
    game_table: Dict[ImpartialGame, int] = {}

    def __init__(self) -> None:
        pass

    def get_nimber(self, game: ImpartialGame) -> int:
        if game in self.game_table:
            return self.game_table[game]
        else:
            moves = (self.get_nimber(move) for move in game.get_moves())
            nimber = mex(moves)
            self.game_table[game] = nimber
            return nimber


class SplittingSolver:
    game_table: Dict[ImpartialGame, int] = {}

    def __init__(self) -> None:
        pass

    def get_nimber(self, game: ImpartialGame) -> int:
        if game in self.game_table:
            return self.game_table[game]
        else:
            total_nimber = 0
            for split_game in game.split():
                if split_game in self.game_table:
                    total_nimber ^= self.game_table[split_game]
                else:
                    moves = (self.get_nimber(move) for move in split_game.get_moves())
                    split_nimber = mex(moves)
                    self.game_table[split_game] = split_nimber
                    total_nimber ^= split_nimber
            self.game_table[game] = total_nimber
            return total_nimber
