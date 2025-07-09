extends Node2D

class_name PointView

signal mouse_entered
signal mouse_exited
signal clicked

@onready var controller : PointController = $".."

func _ready():
	# Setup input event connections if needed
	pass

func _on_mouse_entered():
	emit_signal("mouse_entered")

func _on_mouse_exited():
	emit_signal("mouse_exited")

func _input_event(viewport, event, shape_idx):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		emit_signal("clicked")

func update():
	#do smthng to sprites etc...
	pass
