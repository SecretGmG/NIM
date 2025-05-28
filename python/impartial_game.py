from abc import ABC, abstractmethod
from typing import Self, Iterable

class ImpartialGame(ABC):
    def __init__(self) -> None:
        super().__init__()

    @abstractmethod
    def max_nimber(self):
        """Return the maximum nimber for the game."""
        pass

    @abstractmethod
    def get_moves(self) -> Iterable[Self]:
        """Generate all possible moves for the game."""
        pass

    @abstractmethod
    def split(self) -> Iterable[Self]:
        """Split the game into independent subgames."""
        pass

    @classmethod
    @abstractmethod
    def is_isomorphic(cls, a, b) -> bool:
        """Check if two game states are isomorphic."""
        pass
