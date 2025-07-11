extends Node2D

class_name EditorController

signal level_changed()

# Reference to the BoardController for modifying the board
@onready var board_controller: BoardController = $BoardController

enum Mode {
	Idle,
		# 1. Add points by clicking on background
		#    (indicated by a small grayed-out preview point at the mouse position)
		# 2. Select point by clicking it (indicated by focus highlight)
		# 3. Select line by clicking it (indicated by focus highlight)
	PointSelected,
		# 1. Move by dragging
		#    a) Delete by dragging out of the level bounds
		#    b) Add to a line by dragging onto it (indicated by line focus and snapping)
		# 2. Delete by pressing Del, Backspace, or via button
		# 3. Unselect by clicking line or background
	LineSelected,
		# 1. Delete by pressing Del, Backspace, or via button
		# 2. Add existing points to the line by clicking points (no new points allowed)
		# 3. Unselect by clicking point or background
	AddLine,
		# Entered via button
		# A preview of the line is always rendered (similar to add-point preview)
		# 1. Add points to the new line by clicking existing points
		# 2. Finish via Enter, Esc, or button
		# 3. Cancel via Del, Backspace, or button
}

var mode: Mode = Mode.Idle
var selected_point_id: int = -1
var selected_line_id: int = -1

# For AddLine mode, keep track of points selected to build the line
var control_point_id: int = -1
var add_line_points: Array = []

func _ready():
	pass

func clear_mode_state():
	# Update visual hints, previews, highlights depending on mode
	match mode:
		Mode.Idle:
			pass
		Mode.PointSelected:
			pass
		Mode.LineSelected:
			pass
		Mode.AddLine:
			pass
	selected_point_id = -1
	selected_line_id = -1
	add_line_points.clear()

func activate_idle():
	clear_mode_state()
	self.mode = Mode.Idle
	pass #ui updates

func activate_point_selected(point_id):
	clear_mode_state()
	self.mode = Mode.PointSelected
	self.selected_point_id = point_id
	pass #ui updates

func activate_line_selected(line_id):
	clear_mode_state()
	self.mode = Mode.LineSelected
	self.selected_line_id = line_id
	pass #ui updates

func add_point_to_line(point_id):
	assert(self.mode == Mode.AddLine)
	if add_line_points.has(point_id):
		return # prevent duplicates (maybe allow duplicates shouldn't affect game and allows nice viusuals
	add_line_points.append(point_id)
	# Optional: validate line (e.g. min 2 points), update preview line


func on_background_click(position: Vector2) -> void:
	match mode:
		Mode.Idle:
			var point_id = board_controller.add_point(position)
			activate_point_selected(point_id)
		
		Mode.PointSelected, Mode.LineSelected:
			activate_idle()
		Mode.AddLine:
			# Possibly ignore or finalize add line if clicked outside points
			# Maybe even add point and then this point to line
			pass

func toggle_point_state(point_id : int) -> void:
	match board_controller.get_point(point_id).state:
		PointState.DEFAULT:
			board_controller.set_point_state(point_id, PointState.ACTIVATED)
		PointState.ACTIVATED:
			board_controller.set_point_state(point_id, PointState.CONTROL_VISIBLE)
		PointState.CONTROL_VISIBLE:
			board_controller.set_point_state(point_id, PointState.DEFAULT)

func on_point_click(point_id: int) -> void:
	match mode:
		Mode.Idle:
			activate_point_selected(point_id)
		Mode.PointSelected:
			if selected_point_id == point_id:
				toggle_point_state(point_id)
			else:
				activate_point_selected(point_id)
		Mode.LineSelected:
			activate_point_selected(point_id)
		Mode.AddLine:
			# Add point to line preview
			add_point_to_line(point_id)

func on_line_click(line_id: int) -> void:
	match mode:
		Mode.Idle:
			activate_line_selected(line_id)
		Mode.PointSelected:
			activate_line_selected(line_id)
		Mode.LineSelected:
			if selected_line_id == line_id:
				activate_idle()
			else:
				activate_line_selected(line_id)
		Mode.AddLine:
			# Maybe ignore or cancel add line
			pass

func on_point_drag(point_id: int, new_position: Vector2) -> void:
	if mode != Mode.PointSelected or selected_point_id != point_id:
		return
	# Move point in board controller
	board_controller.move_point(point_id, new_position)

func on_delete_pressed() -> void:
	match mode:
		Mode.PointSelected:
			board_controller.delete_point(selected_point_id)
			activate_idle()
		Mode.LineSelected:
			board_controller.delete_line(selected_line_id)
			activate_idle()
		Mode.AddLine:
			cancel_add_line()

func on_point_hover(point_id: int) -> void:
	# highlight point
	pass

func on_point_unhover(point_id: int) -> void:
	# remove highlight
	pass

func on_line_hover(line_id: int) -> void:
	# highlight line
	pass

func on_line_unhover(line_id: int) -> void:
	# remove highlight
	pass


func cancel_add_line():
	pass
