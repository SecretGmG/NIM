extends Node2D

class_name LineController

signal exited(id : int)
signal entered(id : int)
signal clicked(id : int)

@onready var view : LineView = $LineView

var id : int
var controller : BoardController

var visual_info : VisualInfo = VisualInfo.new()
var point_ids : Array[int] = []

func init(id : int, controller : BoardController) -> LineController:
	self.id = id
	self.controller = controller
	return self

func _ready():
	view.mouse_entered.connect(_on_view_mouse_entered)
	view.mouse_exited.connect(_on_view_mouse_exited)
	view.clicked.connect(_on_view_clicked)
	controller.point_moved.connect(_on_point_moved)

func _on_point_moved(id : int):
	if point_ids.has(id):
		self.view.update()

func insert_point(point_id, idx):
	self.point_ids.insert(idx, point_id)
	self.view.update()

func remove_point(idx):
	self.point_ids.remove_at(idx)
	self.view.update()

func _on_view_mouse_entered():
	entered.emit(id)

func _on_view_mouse_exited():
	exited.emit(id)

func _on_view_clicked():
	emit_signal("clicked", id)

func get_points_positions() -> Array[Vector2]:
	var positions : Array[Vector2] = []
	for point_id in self.point_ids:
		positions.append(self.controller.get_point(point_id).position)
	return positions
