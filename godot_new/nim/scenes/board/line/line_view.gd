extends Node2D

class_name LineView

signal mouse_entered
signal mouse_exited
signal clicked

@onready var area : Area2D = $Area2D
@onready var collision_polygon : CollisionPolygon2D = $Area2D/CollisionPolygon2D
@onready var controller : LineController = $".."

@export var width : float = 10
@export_color_no_alpha var base_color : Color = Color.DARK_GRAY
@export_color_no_alpha var  focused_color : Color = Color.DARK_SLATE_GRAY

var color : Color = base_color
var curve : Curve2D = Curve2D.new()
var curve_polygon : PackedVector2Array = PackedVector2Array()

func _ready():
	area.mouse_entered.connect(_on_mouse_entered)
	area.mouse_exited.connect(_on_mouse_exited)
	area.input_event.connect(_on_input_event)

func _on_mouse_entered():
	mouse_entered.emit()

func _on_mouse_exited():
	mouse_exited.emit()

func _on_input_event(_viewport, event, _shape_idx):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		clicked.emit()

func compute_handle_pair(i : int, positions: Array[Vector2]) -> Array[Vector2]:
	var p0 = positions[clamp(i - 1, 0, positions.size() - 1)]
	var p1 = positions[i]
	var p2 = positions[clamp(i + 1, 0, positions.size() - 1)]

	var dir = (p2 - p0).normalized()
	var len1 = (p1 - p0).length()
	var len2 = (p2 - p1).length()
	var min_len = min(len1, len2)
	
	var factor = 0.3
	
	var out_handle = factor * dir * min_len #* len2
	var in_handle = -factor * dir * min_len #* len1

	return [in_handle, out_handle]

func update():
	if controller.visual_info.focused:
		color = focused_color
	else:
		color = base_color
	
	update_curve()
	queue_redraw()

func update_curve():
	curve.clear_points()
	
	var positions = controller.get_points_positions()
	
	for i in range(positions.size()):
		var handles = compute_handle_pair(i, positions)
		curve.add_point(positions[i], handles[0], handles[1])
	
	self.curve_polygon = get_curve_polygon(self.width)
	self.collision_polygon.set_polygon(self.curve_polygon)
	self.queue_redraw()

func get_curve_polygon(_width : float) -> PackedVector2Array:
	var offset = _width * 0.5
	var left : PackedVector2Array = []
	var right : PackedVector2Array = []
	
	var points = curve.tessellate()
	
	if points.size() < 2:
		return PackedVector2Array()

	for i in range(points.size()):
		var dir = Vector2.ZERO
		if i < points.size() - 1:
			dir += (points[i + 1] - points[i]).normalized().orthogonal()
		if i > 0:
			dir += (points[i] - points[i - 1]).normalized().orthogonal()
		dir = dir.normalized() * offset

		left.append(points[i] + dir)
		right.append(points[i] - dir)

	right.reverse()
	var polygon = left+right
	return polygon

func _draw():
	self.draw_colored_polygon(self.curve_polygon, self.color)
