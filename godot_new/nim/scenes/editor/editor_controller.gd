extends Node

class_name EditorController

signal level_changed()

# Reference to the BoardController for modifying the board
var board_controller: Node = null

func _ready():
	# Setup editor UI, connect signals, prepare editing state
	pass

func add_line():
	# Add a new line to the board
	pass

func add_point(position: Vector2):
	# Add a new point at the given position
	pass

func move_point(point_id, new_position: Vector2):
	# Update position of a point or control point
	pass

func delete_point(point_id):
	# Delete a point or control point
	pass

func delete_line(line_id):
	# Delete a line and update board accordingly
	pass

func toggle_point_activation(point_id, activated: bool):
	# Used to toggle activation state in editor for testing
	pass
