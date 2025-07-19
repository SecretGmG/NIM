extends Control

class_name BoardEditorView

signal background_pressed(pos : Vector2)
signal add_line_pressed
signal save_game(path : String)
signal load_game(path : String)
signal delete_pressed
signal escape_pressed
signal play_pressed


@onready var background : ColorRect = $Background
@onready var escape_button : Button = $HBoxContainer/EscapeButton
@onready var delete_button : Button = $HBoxContainer/DeleteButton
@onready var add_line_button : Button = $HBoxContainer/AddLineButton
@onready var save_game_button : Button = $HBoxContainer/SaveGameButton
@onready var load_game_button : Button = $HBoxContainer/LoadGameButton
@onready var play_game_button : Button = $HBoxContainer/PlayGameButton

@onready var save_game_file_dialog : FileDialog = $SaveGameFileDialog
@onready var load_game_file_dialog : FileDialog = $LoadGameFileDialog

func _ready() -> void:
	background.gui_input.connect(_on_gui_input)
	
	escape_button.pressed.connect(_on_escape_button_pressed)
	delete_button.pressed.connect(_on_delete_button_pressed)
	add_line_button.pressed.connect(_on_add_line_button_pressed)
	save_game_button.pressed.connect(_on_save_game_button_pressed)
	load_game_button.pressed.connect(_on_load_game_button_pressed)
	play_game_button.pressed.connect(_on_play_game_button_pressed)
	save_game_file_dialog.file_selected.connect(_on_save_file_selected)
	load_game_file_dialog.file_selected.connect(_on_load_file_selected)

func _process(_delta: float) -> void:
	background.set_global_position(get_viewport_rect().position)
	background.set_size(get_viewport_rect().size)


func _input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_escape"):
		escape_pressed.emit()
	if event.is_action_pressed("ui_delete"):
		delete_pressed.emit()
	if event.is_action_pressed("ui_add_line"):
		add_line_pressed.emit()
	if event.is_action_pressed("ui_save"):
		_on_save_game_button_pressed()

func _on_gui_input(event: InputEvent):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		background_pressed.emit(get_global_mouse_position())

func _on_escape_button_pressed():
	escape_pressed.emit()

func _on_delete_button_pressed():
	delete_pressed.emit()

func _on_add_line_button_pressed():
	add_line_pressed.emit()

func _on_save_game_button_pressed():
	save_game_file_dialog.show()

func _on_save_file_selected(path):
	save_game.emit(path)
	save_game_file_dialog.hide()
	
func _on_load_game_button_pressed():
	load_game_file_dialog.show()

func _on_load_file_selected(path):
	load_game.emit(path)
	load_game_file_dialog.hide()

func _on_play_game_button_pressed():
	play_pressed.emit()
	
