extends Node2D

class_name BoardController

const POINT_SCENE = preload("res://scenes/board/point/Point.tscn")
const LINE_SCENE = preload("res://scenes/board/line/Line.tscn")

signal point_entered(point_id : int)
signal line_entered(line_id : int)
signal point_exited(point_id : int)
signal line_exited(line_id : int)
signal point_clicked(point_id : int)
signal line_clicked(line_id : int)

signal point_moved(point_id : int)

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

func _on_point_clicked(point_id):
	point_clicked.emit(point_id)

func _on_line_entered(line_id):
	line_entered.emit(line_id)

func _on_line_exited(line_id):
	line_exited.emit(line_id)

func _on_line_clicked(line_id):
	line_clicked.emit(line_id)

func get_lines_for_point(point_id) -> Array[int]:
	var lines_for_point : Array[int] = []
	for line_id in self.lines.keys():
		if lines[line_id].point_ids.has(point_id):
			lines_for_point.append(line_id)
	return lines_for_point

func insert_point_to_line(point_id, line_id, idx):
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

func spawn_point() -> int:
	var point = POINT_SCENE.instantiate().init(next_point_id, self)
	add_child(point)
	point.exited.connect(_on_point_exited)
	point.entered.connect(_on_point_entered)
	point.clicked.connect(_on_point_clicked)
	point.moved.connect(_on_point_moved)
	points[next_point_id] = point
	next_point_id = next_point_id + 1
	return point.id

func spawn_line() -> int:
	var line = LINE_SCENE.instantiate().init(next_line_id, self)
	add_child(line)
	line.exited.connect(_on_line_exited)
	line.entered.connect(_on_line_entered)
	line.clicked.connect(_on_line_clicked)
	lines[next_line_id] = line
	next_line_id = next_line_id + 1
	return line.id
