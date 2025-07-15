extends Node2D

class_name LineController

signal exited(id : int)
signal entered(id : int)
signal pressed(id : int)

@onready var view : LineView = $LineView

var id : int
var controller : BoardController

var visual_info : VisualInfo = VisualInfo.new()
var point_ids : Array[int] = []

func init(_id : int, _controller : BoardController) -> LineController:
	self.id = _id
	self.controller = _controller
	return self

func _ready():
	view.mouse_entered.connect(_on_mouse_entered)
	view.mouse_exited.connect(_on_mouse_exited)
	view.pressed.connect(_on_view_pressed)

func _on_point_moved(_id : int):
	if point_ids.has(_id):
		self.view.update()

func insert_point(point_id, idx):
	if idx == -1:
		self.point_ids.append(point_id)
	else:
		self.point_ids.insert(idx, point_id)
	self.view.update()

func remove_point(idx):
	self.point_ids.remove_at(idx)
	self.view.update()

func _on_mouse_entered():
	entered.emit(id)

func _on_mouse_exited():
	exited.emit(id)

func _on_view_pressed():
	pressed.emit(id)

func get_points_positions() -> Array[Vector2]:
	var positions : Array[Vector2] = []
	for point_id in self.point_ids:
		positions.append(self.controller.get_point(point_id).position)
	return positions


func set_visual_info(_visual_info : VisualInfo):
	self.visual_info = _visual_info
	view.update()
	
