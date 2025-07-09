extends Node2D

@onready var board : BoardController = $BoardController



func _ready() -> void:
	var point1_id = board.spawn_point()
	board.get_point(point1_id).global_position = get_viewport_rect().get_center()
	
	var point2_id = board.spawn_point()
	board.get_point(point2_id).global_position = get_viewport_rect().get_center() + get_viewport_rect().size * 0.25
	
	var line_id = board.spawn_line()
	board.insert_point_to_line(point1_id, line_id, 0)
	board.insert_point_to_line(point2_id, line_id, 1)
	
	
	
	
