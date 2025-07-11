extends Node2D

class_name GameController

@onready var board: BoardController = $BoardController

var active_point := -1
var focused_point := -1

func _ready():
	# Connect board-level signals
	board.point_clicked.connect(_on_point_clicked)
	board.line_clicked.connect(_on_line_clicked)
	board.point_entered.connect(_on_point_entered)
	board.line_entered.connect(_on_line_entered)
	board.point_exited.connect(_on_point_exited)
	board.line_exited.connect(_on_line_exited)

	# TEMP: Initialize test board (can be refactored later)
	_test_setup()


func _test_setup():
	var p1 = board.spawn_point()
	var p2 = board.spawn_point()

	board.get_point(p1).global_position = get_viewport_rect().get_center()
	board.get_point(p2).global_position = get_viewport_rect().get_center() + get_viewport_rect().size * 0.25

	var l = board.spawn_line()
	board.insert_point_to_line(p1, l, 0)
	board.insert_point_to_line(p2, l, 1)


# Signal callbacks

func _on_point_entered(id: int):
	if focused_point != -1:
		board.set_point_focused(focused_point, false)
	focused_point = id
	board.set_point_focused(focused_point, true)

func _on_point_exited(id: int):
	if id == focused_point:
		board.set_point_focused(focused_point, false)
		focused_point = -1

func _on_point_clicked(id: int):
	# Implement activation/deactivation or selection logic
	print("Point clicked: %d" % id)
	# Example: toggle activation
	if active_point == id:
		board.set_point_selected(id, false)
		active_point = -1
	else:
		if active_point != -1:
			board.set_point_selected(active_point, false)
		board.set_point_selected(id, true)
		active_point = id

func _on_line_entered(id: int):
	# Optional: line hover logic
	pass

func _on_line_exited(id: int):
	# Optional: line hover logic
	pass

func _on_line_clicked(id: int):
	# Optional: handle clicking on lines
	print("Line clicked: %d" % id)
