extends Control

class_name PlayerView

signal background_pressed(pos : Vector2)
signal escape_pressed
signal make_move_pressed
signal make_bot_move_pressed
signal go_to_editor_pressed

@onready var background : ColorRect = $Background
@onready var escape_button : Button = $HBoxContainer/EscapeButton
@onready var make_move_button : Button = $HBoxContainer/MakeMoveButton
@onready var make_bot_move_button : Button = $HBoxContainer/MakeBotMoveButton
@onready var go_to_editor_button : Button = $HBoxContainer/GoToEditorButton

func _ready() -> void:
	background.gui_input.connect(_on_gui_input)
	escape_button.pressed.connect(_on_escape_button_pressed)
	make_move_button.pressed.connect(_on_make_move_button_pressed)
	make_bot_move_button.pressed.connect(_on_make_bot_move_button_pressed)
	go_to_editor_button.pressed.connect(_on_go_to_editor_button_pressed)

func _process(_delta: float) -> void:
	background.set_global_position(get_viewport_rect().position)
	background.set_size(get_viewport_rect().size)

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_escape"):
		escape_pressed.emit()

func _on_gui_input(event: InputEvent):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		background_pressed.emit(get_global_mouse_position())
	

func _on_escape_button_pressed():
	escape_pressed.emit()
	
func _on_make_move_button_pressed():
	make_move_pressed.emit()

func _on_make_bot_move_button_pressed():
	make_bot_move_pressed.emit()

func _on_go_to_editor_button_pressed():
	go_to_editor_pressed.emit()
