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

func init(id : int, controller : BoardController) -> PointController:
	self.id = id
	self.controller = controller
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


func set_state(state: int):
	self.state = state
	view.update()

func set_visual_info(visual_info : VisualInfo):
	self.visual_info = visual_info
	view.update()
	
