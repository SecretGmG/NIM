extends Node2D

class_name LineView

signal mouse_entered
signal mouse_exited
signal clicked

@onready var controller : LineController = $".."

var width : float = 10
var color : Color = Color.RED
var curve : Curve2D = Curve2D.new()

func _ready():
	pass

func _on_mouse_entered():
	emit_signal("mouse_entered")

func _on_mouse_exited():
	emit_signal("mouse_exited")

func _input_event(viewport, event, shape_idx):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		emit_signal("clicked")

func update():
	self.curve.clear_points()
	for point in controller.get_points_positions():
		# TODO compute in and out points
		var _in = Vector2.ZERO
		var out = Vector2.ZERO
		curve.add_point(point, _in, out)
	
	# TODO: Update width and color from state
	queue_redraw()

func _draw():
	# draw bezier curve with control points
	var points = self.curve.tessellate()
	for i in range(len(points)-1):
		draw_line(points[i], points[i+1], self.color, self.width)
