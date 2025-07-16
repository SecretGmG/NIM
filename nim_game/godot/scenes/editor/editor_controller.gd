extends Node2D

class_name BoardEditorController

@onready var board_controller : BoardController = $BoardController
@onready var view : BoardEditorView = $CanvasLayer/EditorView

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
		# 4. Toglle state by clicking if it was not dragged
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

var focused_point_id: int = -1
var focused_line_id: int = -1

# For AddLine mode, keep track of points selected to build the line
var control_point_id: int = -1
var add_line_line_id: int = -1

# For PointSelected mode, keep track if point is dragged

var is_dragging : bool = false
var toggle_on_release : bool = false

func _ready():
	# Connect point signals
	board_controller.point_pressed.connect(on_point_pressed)
	board_controller.point_released.connect(on_point_released)
	board_controller.point_entered.connect(on_point_entered)
	board_controller.point_exited.connect(on_point_exited)
	board_controller.point_deleted.connect(on_point_deleted)

	# Connect line signals
	board_controller.line_pressed.connect(on_line_pressed)
	board_controller.line_entered.connect(on_line_entered)
	board_controller.line_exited.connect(on_line_exited)
	board_controller.line_deleted.connect(on_line_deleted)
	
	# Connect view signals
	view.background_pressed.connect(on_background_pressed)
	
	view.add_line_button_pressed.connect(activate_add_line)
	view.save_game_button_pressed.connect(save_game)
	view.load_game_button_pressed.connect(load_game)
	
func _input(event: InputEvent) -> void:
	if event.is_action("ui_delete"):
		on_delete_pressed()
	if event.is_action("ui_escape"):
		on_escape_pressed()
	if event.is_action("print_debug_info"):
		if add_line_line_id != -1:
			print(board_controller.get_line(add_line_line_id).point_ids)
	
	
	# Handle drag
	if mode == Mode.PointSelected and selected_point_id != -1:
		if event is InputEventMouseMotion and is_dragging:
			var pos = get_global_mouse_position()
			board_controller.move_point(selected_point_id, pos)
			toggle_on_release = false
	
	if mode == Mode.AddLine and control_point_id != -1:
		if event is InputEventMouseMotion:
			var pos = get_global_mouse_position()
			board_controller.move_point(control_point_id, pos)


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
			if control_point_id != -1:
				board_controller.remove_point(self.control_point_id)
	
	is_dragging = false
	selected_point_id = -1
	selected_line_id = -1
	control_point_id = -1
	add_line_line_id = -1

func activate_idle():
	clear_mode_state()
	self.mode = Mode.Idle
	print('idle activated')
	pass #ui updates

func activate_point_selected(point_id):
	clear_mode_state()
	self.mode = Mode.PointSelected
	self.selected_point_id = point_id
	self.is_dragging = true
	self.toggle_on_release = false
	print('point selected activated')
	pass #ui updates

func activate_line_selected(line_id):
	clear_mode_state()
	self.mode = Mode.LineSelected
	self.selected_line_id = line_id
	print('line selected activated')
	pass #ui updates

func activate_add_line():
	clear_mode_state()
	self.mode = Mode.AddLine
	control_point_id = board_controller.spawn_point(PointState.CONTROL_HIDDEN)
	add_line_line_id = board_controller.spawn_line()
	board_controller.insert_point_to_line(control_point_id, add_line_line_id)
	print('add line activated')


func on_background_pressed(_position: Vector2) -> void:
	print('background pressed')
	if focused_line_id != -1 or focused_point_id != -1:
		return
	match mode:
		Mode.Idle:
			var point_id = board_controller.spawn_point()
			board_controller.move_point(point_id, _position)
			activate_point_selected(point_id)
		Mode.PointSelected, Mode.LineSelected:
			activate_idle()
		Mode.AddLine:
			var new_point_id = control_point_id
			control_point_id = board_controller.spawn_point(PointState.CONTROL_HIDDEN)
			board_controller.insert_point_to_line(control_point_id, add_line_line_id)
			board_controller.move_point(control_point_id, _position)
			board_controller.set_point_state(new_point_id, PointState.DEFAULT)

func toggle_point_state(point_id : int) -> void:
	match board_controller.get_point(point_id).state:
		PointState.DEFAULT:
			board_controller.set_point_state(point_id, PointState.ACTIVATED)
		PointState.ACTIVATED:
			board_controller.set_point_state(point_id, PointState.CONTROL_VISIBLE)
		PointState.CONTROL_VISIBLE:
			board_controller.set_point_state(point_id, PointState.DEFAULT)

func on_point_pressed(point_id: int) -> void:
	print('point pressed')
	match mode:
		Mode.Idle:
			activate_point_selected(point_id)
		Mode.PointSelected:
			if self.selected_point_id == point_id:
				is_dragging = true
			else:
				activate_point_selected(point_id)
		Mode.LineSelected:
			activate_point_selected(point_id)
		Mode.AddLine:
			var line = board_controller.get_line(add_line_line_id)
			if not point_id in line.point_ids:
				var idx = len(line.point_ids) - 1
				board_controller.insert_point_to_line(point_id, add_line_line_id, idx)

func on_point_released(point_id: int) -> void:
	print('point released')
	match mode:
		Mode.PointSelected:
			if toggle_on_release:
				toggle_point_state(point_id)
			else:
				toggle_on_release = true
			is_dragging = false

func on_point_deleted(point_id : int) -> void:
	if selected_point_id == point_id:
		selected_line_id = -1
	if focused_point_id == point_id:
		focused_point_id = -1
	if control_point_id ==  point_id:
		control_point_id = -1

func on_line_deleted(line_id : int) -> void:
	if selected_line_id == line_id:
		selected_line_id = -1
	if focused_line_id == line_id:
		focused_line_id = -1
	if add_line_line_id ==  line_id:
		add_line_line_id = -1

func on_line_pressed(line_id: int) -> void:
	print('line pressed, focused pointid : {0}'.format([focused_point_id]))
	if focused_point_id != -1:
		# points are 'in front of' lines
		# if a point is focused line clicks will be ignored
		return
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

func on_delete_pressed() -> void:
	match mode:
		Mode.PointSelected:
			board_controller.remove_point(selected_point_id)
			activate_idle()
		Mode.LineSelected:
			board_controller.remove_line(selected_line_id)
			activate_idle()
		Mode.AddLine:
			cancel_add_line()

func on_escape_pressed() -> void:
	match mode:
		Mode.PointSelected:
			activate_idle()
		Mode.LineSelected:
			activate_idle()
		Mode.AddLine:
			activate_idle()


func on_point_entered(point_id: int) -> void:
	print('point entered')
	if point_id == control_point_id:
		return #ignore the control point
	if focused_point_id != point_id:
		if focused_point_id != -1:
			board_controller.set_point_focused(focused_point_id, false)
		if focused_line_id != -1:
			board_controller.set_line_focused(focused_line_id, false)
		board_controller.set_point_focused(point_id, true)
		focused_point_id = point_id

func on_point_exited(point_id: int) -> void:
	print('point exited')
	if focused_point_id == point_id:
		board_controller.set_point_focused(point_id, false)
		focused_point_id = -1
		is_dragging = false
	if focused_line_id != -1:
		board_controller.set_line_focused(focused_line_id, true)

func on_line_entered(line_id: int) -> void:
	if focused_line_id != line_id:
		if focused_line_id != -1:
			board_controller.set_line_focused(focused_line_id, false)
		if focused_point_id == -1:
			board_controller.set_line_focused(line_id, true)
		focused_line_id = line_id

func on_line_exited(line_id: int) -> void:
	if focused_line_id == line_id:
		board_controller.set_line_focused(line_id, false)
		focused_line_id = -1


func cancel_add_line():
	board_controller.remove_line(self.add_line_line_id)
	add_line_line_id = -1
	clear_mode_state()
	activate_idle()


func save_game():
	var state = board_controller.get_state()
	var err = ResourceSaver.save(state, 'res://assets/boards/test_board.res')
	print('saved game')
	print(err)

func load_game():
	
	var state = load("res://assets/boards/test_board.res")
	board_controller.load_state(state)
	print('loaded game')
