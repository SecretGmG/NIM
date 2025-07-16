extends Node2D

class_name BoardController

const POINT_SCENE = preload("res://scenes/board/point/Point.tscn")
const LINE_SCENE = preload("res://scenes/board/line/Line.tscn")

signal point_entered(point_id : int)
signal line_entered(line_id : int)
signal point_exited(point_id : int)
signal line_exited(line_id : int)
signal point_pressed(point_id : int)
signal point_released(point_id : int)
signal line_pressed(line_id : int)

signal line_deleted(line_id : int)
signal point_deleted(point_id : int)

# Core board state: points, lines, and their relationships
var next_point_id : int = 0
var next_line_id : int = 0

var points : Dictionary[int,PointController] = {}
var lines : Dictionary[int, LineController] = {}

func _ready():
	# Setup signals, initialize board data
	pass

func _on_point_moved(point_id):
	for line_id in self.get_lines_for_point(point_id):
		self.get_line(line_id).view.update()

func _on_point_entered(point_id):
	point_entered.emit(point_id)

func _on_point_exited(point_id):
	point_exited.emit(point_id)

func _on_point_pressed(point_id):
	point_pressed.emit(point_id)

func _on_point_released(point_id):
	point_released.emit(point_id)

func _on_line_entered(line_id):
	line_entered.emit(line_id)

func _on_line_exited(line_id):
	line_exited.emit(line_id)

func _on_line_pressed(line_id):
	line_pressed.emit(line_id)

func get_lines_for_point(point_id) -> Array[int]:
	var lines_for_point : Array[int] = []
	for line_id in self.lines.keys():
		if lines[line_id].point_ids.has(point_id):
			lines_for_point.append(line_id)
	return lines_for_point

func remove_point(point_id):
	points[point_id].queue_free()
	points.erase(point_id)
	for line_id in lines.keys():
		if lines[line_id].point_ids.has(point_id):
			remove_point_from_line(line_id, lines[line_id].point_ids.find(point_id))
			if lines[line_id].point_ids.size() < 2:
				remove_line(line_id)
	point_deleted.emit(point_id)

func remove_line(line_id):
	lines[line_id].queue_free()
	lines.erase(line_id)
	line_deleted.emit(line_id)

func move_point(point_id, _global_position):
	points[point_id].global_position = _global_position
	for line_id in lines.keys():
		if lines[line_id].point_ids.has(point_id):
			lines[line_id].view.update()

func insert_point_to_line(point_id, line_id, idx = -1):
	self.lines[line_id].insert_point(point_id, idx)

func remove_point_from_line(line_id, idx):
	self.lines[line_id].remove_point(idx)

func get_point_id(point : PointController) -> int:
	return points.find_key(point)

func get_line_id(line : LineController) -> int:
	return lines.find_key(line)

func get_point(point_id) -> PointController:
	return points[point_id]

func get_line(line_id) -> LineController:
	return lines[line_id]

func set_line_focused(line_id, focused):
	var vi : VisualInfo = lines[line_id].visual_info
	vi.focused = focused
	lines[line_id].set_visual_info(vi)

func set_line_possible_to_select(line_id, possible_to_select):
	var vi : VisualInfo = lines[line_id].visual_info
	vi.possible_to_select = possible_to_select
	lines[line_id].set_visual_info(vi)

func set_line_selected(line_id, selected):
	var vi : VisualInfo = lines[line_id].visual_info
	vi.selected = selected
	lines[line_id].set_visual_info(vi)

func set_point_focused(point_id, focused):
	var vi : VisualInfo = points[point_id].visual_info
	vi.focused = focused
	points[point_id].set_visual_info(vi)
	
func set_point_possible_to_select(point_id, possible_to_select):
	var vi : VisualInfo = points[point_id].visual_info
	vi.possible_to_select = possible_to_select
	points[point_id].set_visual_info(vi)

func set_point_selected(point_id, selected):
	var vi : VisualInfo = points[point_id].visual_info
	vi.selected = selected
	points[point_id].set_visual_info(vi)

func set_point_state(point_id, state : int):
	points[point_id].set_state(state)

func spawn_point(state : int = PointState.DEFAULT) -> int:
	var point = POINT_SCENE.instantiate().init(next_point_id, self, state)
	add_child(point)
	point.exited.connect(_on_point_exited)
	point.entered.connect(_on_point_entered)
	point.pressed.connect(_on_point_pressed)
	point.released.connect(_on_point_released)
	point.moved.connect(_on_point_moved)
	points[next_point_id] = point
	next_point_id = next_point_id + 1
	return point.id

func spawn_line() -> int:
	var line = LINE_SCENE.instantiate().init(next_line_id, self)
	add_child(line)
	line.exited.connect(_on_line_exited)
	line.entered.connect(_on_line_entered)
	line.pressed.connect(_on_line_pressed)
	lines[next_line_id] = line
	next_line_id = next_line_id + 1
	return line.id

func clear_board() -> void:
	for line_id in self.lines.keys():
		self.remove_line(line_id)
	for point_id in self.points.keys():
		self.remove_point(point_id)


func get_state() -> BoardState:
	var state := BoardState.new()

	# Ensure a consistent point order
	var sorted_point_ids := points.keys()
	sorted_point_ids.sort()

	state.point_positions = PackedVector2Array(
		sorted_point_ids.map(func(id): return points[id].position)
	)
	state.point_states = PackedInt32Array(
		sorted_point_ids.map(func(id): return points[id].state)
	)
	
	var points_in_lines : Array[PackedInt32Array] = []
	# Store lines as list of point indices in the sorted list
	for line in lines.values():
		var point_indices := []
		for pid in line.point_ids:
			point_indices.append(sorted_point_ids.find(pid))
		points_in_lines.append(PackedInt32Array(point_indices))
	
	state.points_in_lines = points_in_lines
	
	return state


func load_state(state: BoardState) -> void:
	clear_board()

	var id_map := []
	
	# Recreate points and record mapping
	for i in range(state.point_states.size()):
		var point_id := spawn_point(state.point_states[i])
		get_point(point_id).position = state.point_positions[i]
		id_map.append(point_id)

	# Recreate lines using mapped point IDs
	for points_in_line in state.points_in_lines:
		var line_id := spawn_line()
		for index in points_in_line:
			insert_point_to_line(id_map[index], line_id)
