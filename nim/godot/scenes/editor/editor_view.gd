extends Control

class_name BoardEditorView

signal background_pressed
signal add_line_button_pressed
signal save_game_button_pressed
signal load_game_button_pressed

@onready var background : ColorRect = $Background
@onready var add_line_button : Button = $HBoxContainer/AddLineButton
@onready var save_game_button : Button = $HBoxContainer/SaveGameButton
@onready var load_game_button : Button = $HBoxContainer/LoadGameButton

func _ready() -> void:
	background.gui_input.connect(_on_gui_input)
	add_line_button.pressed.connect(_on_add_line_button_pressed)
	save_game_button.pressed.connect(_on_save_game_button_pressed)
	load_game_button.pressed.connect(_on_load_game_button_pressed)

func _process(_delta: float) -> void:
	background.set_global_position(get_viewport_rect().position)
	background.set_size(get_viewport_rect().size)

func _on_gui_input(event: InputEvent):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		background_pressed.emit(get_global_mouse_position())

func _on_add_line_button_pressed():
	add_line_button_pressed.emit()

func _on_save_game_button_pressed():
	save_game_button_pressed.emit()
	
func _on_load_game_button_pressed():
	load_game_button_pressed.emit()
	
