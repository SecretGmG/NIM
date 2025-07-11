extends Node2D

class_name PointController

signal entered(id : int)
signal exited(id : int)
signal clicked(id : int)
signal moved(id : int)

var id : int
var controller : BoardController

var state : int = PointState.DEFAULT
var visual_info : VisualInfo = VisualInfo.new()

@onready var view : PointView = $PointView

func init(_id : int, _controller : BoardController) -> PointController:
	self.id = _id
	self.controller = _controller
	return self

func _ready():
	view.mouse_entered.connect(_on_view_mouse_entered)
	view.mouse_exited.connect(_on_view_mouse_exited)
	view.clicked.connect(_on_view_clicked)

func _on_view_mouse_entered():
	entered.emit(id)

func _on_view_mouse_exited():
	exited.emit(id)

func _on_view_clicked():
	clicked.emit(id)

func _on_moved():
	moved.emit(id)


func set_state(_state: int):
	self.state = _state
	view.update()

func set_visual_info(_visual_info : VisualInfo):
	self.visual_info = _visual_info
	view.update()
	
