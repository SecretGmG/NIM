extends Node2D

class_name Main

var BOARD_EDITOR_SCENE = preload("res://scenes/editor/BoardEditor.tscn")
var BOARD_PLAYER_SCENE = preload("res://scenes/player/BoardPlayer.tscn")
var DEFAULT_BOARD_STATE = BoardState.new()

var editor_board_state : BoardState

var current_scene : Node

func _ready() -> void:
	editor_board_state = DEFAULT_BOARD_STATE
	set_editor_scene()

func get_current_scene_as_editor() -> BoardEditor:
	return current_scene as BoardEditor

func get_current_scene_as_player() -> BoardPlayer:
	return current_scene as BoardPlayer

func on_play_pressed_in_editor():
	print('play pressed in editor')
	var editor = get_current_scene_as_editor()
	editor_board_state = editor.board_controller.get_state()
	set_game_scene()

func on_go_to_editor_pressed():
	set_editor_scene()

func set_editor_scene():
	if current_scene is Node:
		current_scene.queue_free()
	var editor : BoardEditor = BOARD_EDITOR_SCENE.instantiate()
	editor.play_pressed.connect(self.on_play_pressed_in_editor)
	add_child(editor)
	editor.init(editor_board_state)
	current_scene = editor

func set_game_scene():
	if current_scene is Node:
		current_scene.queue_free()
	var player : BoardPlayer = BOARD_PLAYER_SCENE.instantiate()
	player.go_to_editor_pressed.connect(self.on_go_to_editor_pressed)
	add_child(player)
	player.init(editor_board_state)
	current_scene = player
