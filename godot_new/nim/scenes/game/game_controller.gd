
extends Node

class_name GameController

signal move_made(line_id, activated_point_ids)

# Reference to the BoardController to communicate with the board
var board_controller: Node = null

func _ready():
	# Called when the node enters the scene tree
	# Initialize references, connect signals, etc.
	pass

func on_point_selected(point_id):
	# Handle point selection during the game
	pass

func on_line_selected(line_id):
	# Handle line selection during the game
	pass

func make_move(line_id, activated_points):
	# Validate and apply a move (line chosen, points activated)
	# Emit signal or update state accordingly
	emit_signal("move_made", line_id, activated_points)
