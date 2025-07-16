extends Node2D

class_name GameController

@onready var board_controller: BoardController = $BoardController

var focused_point_id : int = -1
var focused_line_id : int = -1
var selected_point_ids : Array[int] = []

var selectable_point_ids : Array[int] = []

func _ready():
	# Connect point signals
	board_controller.point_pressed.connect(on_point_pressed)
	board_controller.point_released.connect(on_point_released)
	board_controller.point_entered.connect(on_point_entered)
	board_controller.point_exited.connect(on_point_exited)

	# Connect line signals
	board_controller.line_pressed.connect(on_line_pressed)
	board_controller.line_entered.connect(on_line_entered)
	board_controller.line_exited.connect(on_line_exited)

	# TEMP: Initialize test board
	_test_setup()

func _test_setup():
	board_controller.load_state(load('res://assets/boards/test_board.res'))
	update_selectable_points()

func _input(event: InputEvent) -> void:
	if event.is_action("ui_accept"):
		on_make_move()

func update_selectable_points() -> void:
	
	if selected_point_ids.is_empty():
		selectable_point_ids.clear()
		for point_id in board_controller.points.keys():
			if board_controller.get_point(point_id).state == PointState.DEFAULT:
				selectable_point_ids.append(point_id)
		return
	
	var line_candidates = []
	
	for line_id in board_controller.lines.keys():
		var line_is_candidate = true
		for point_id in selected_point_ids:
			if board_controller.get_line(line_id).point_ids.has(point_id):
				continue
			else:
				line_is_candidate = false
				break
		if line_is_candidate:
			line_candidates.append(line_id)
	
	selectable_point_ids.clear()
	
	for line_id in line_candidates:
		for point_id in board_controller.get_line(line_id).point_ids:
			if ((not selectable_point_ids.has(point_id)) and 
				board_controller.get_point(point_id).state == PointState.DEFAULT):
				selectable_point_ids.append(point_id)
		

func can_point_be_selected(point_id: int) -> bool:
	return selectable_point_ids.has(point_id)
	

func toggle_point_selection(point_id: int) -> void:
	if point_id in selected_point_ids:
		# Deselect point
		selected_point_ids.erase(point_id)
		board_controller.set_point_selected(point_id, false)
	else:
		# Select point
		selected_point_ids.append(point_id)
		board_controller.set_point_selected(point_id, true)
	update_selectable_points()

func on_point_pressed(point_id: int) -> void:
	
	if can_point_be_selected(point_id) or point_id in selected_point_ids:
		toggle_point_selection(point_id)

func on_point_released(_point_id : int) -> void:
	pass

func on_point_entered(point_id: int) -> void:
	if focused_point_id != point_id:
		if focused_point_id != -1:
			board_controller.set_point_focused(focused_point_id, false)
		board_controller.set_point_focused(point_id, true)
		focused_point_id = point_id

func on_point_exited(point_id: int) -> void:
	if focused_point_id == point_id:
		board_controller.set_point_focused(point_id, false)
		focused_point_id = -1

func on_make_move() -> void:
	if selected_point_ids.is_empty():
		# Add some UI feedback
		return
	for point_id in selected_point_ids:
		board_controller.set_point_state(point_id, PointState.ACTIVATED)
		board_controller.set_point_selected(point_id, false)
	
	#make ai move or change to other player. for now do nothing
	
	selected_point_ids = []
	update_selectable_points()

func on_line_pressed(_line_id: int) -> void:
	pass
	#maybe toggle selection for all point on this line

func on_line_entered(line_id: int) -> void:
	if focused_line_id != line_id:
		if focused_line_id != -1:
			board_controller.set_line_focused(focused_line_id, false)
		if focused_point_id == -1:
			board_controller.set_line_focused(line_id, true)
		focused_line_id = line_id

func on_line_exited(line_id: int) -> void:
	if focused_line_id == line_id:
		board_controller.set_line_focused(line_id, false)
		focused_line_id = -1
