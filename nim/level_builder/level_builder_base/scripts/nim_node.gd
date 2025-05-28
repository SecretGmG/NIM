extends Node2D
class_name NimNode

var ui_state : UiState = UiState.DEFAULT
var game_state : GameState = GameState.IDLE

func _ready() -> void:
	pass

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


enum UiState {GROUP_HOVER, SELF_HOVER, DEFAULT}
enum GameState {IDLE, ACTIVATED, INERT}
