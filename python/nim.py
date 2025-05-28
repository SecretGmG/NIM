from typing import Iterable, Sequence
from impartial_game import ImpartialGame

class Nim(ImpartialGame):
    cols: Sequence[int]

    def __init__(self, cols: Sequence[int]) -> None:
        self.cols = tuple(cols)  # Ensures immutability of input sequence
        super().__init__()

    def max_nimber(self) -> int:
        """Return the maximum nimber, which is the sum of all columns."""
        return sum(self.cols)

    def get_moves(self) -> Iterable['Nim']:
        """Generate all possible moves by creating a new game state for each valid move."""
        for i, col in enumerate(self.cols):
            for j in range(col):
                new_cols = list(self.cols)
                new_cols[i] = j
                yield Nim(new_cols)

    @classmethod
    def is_isomorphic(cls, a: 'Nim', b: 'Nim') -> bool:
        """Check if two Nim games are isomorphic by comparing sorted columns."""
        return sorted(a.cols) == sorted(b.cols)

    def split(self) -> Iterable['Nim']:
        """Split each column into individual Nim games."""
        return (Nim([col]) for col in self.cols)

    def __str__(self):
        self.cols = tuple(self.cols)
        return self.cols.__str__()
    def __repr__(self):
        return tuple(self.cols).__repr__()
    
    def __hash__(self) -> int:
        self.cols = tuple(self.cols)
        return self.cols.__hash__()
    def __eq__(self, value: object) -> bool:
        self.cols = tuple(self.cols)
        value.cols = tuple(value.cols)
        return self.cols.__eq__(value.cols)
    
    def to_string(self) -> str:
        """Return a string representation of the game state."""
        return self.cols.to_string()
