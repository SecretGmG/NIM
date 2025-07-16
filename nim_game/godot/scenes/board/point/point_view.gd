extends Node2D

class_name PointView

signal mouse_entered
signal mouse_exited
signal pressed
signal released

@onready var controller : PointController = $".."

@onready var state_nodes : Dictionary [int, Node2D] = {
	PointState.DEFAULT: $Default,
	PointState.ACTIVATED: $Activated,
	PointState.CONTROL_HIDDEN: $Hidden,
	PointState.CONTROL_VISIBLE: $Control,
}

var state_radii : Dictionary [int, float] = {
	PointState.DEFAULT: 25,
	PointState.ACTIVATED: 25,
	PointState.CONTROL_HIDDEN: 0,
	PointState.CONTROL_VISIBLE: 12.5,
}

@onready var area2d : Area2D = $Area2D

var current_state : int

func _ready():
	area2d.mouse_entered.connect(_on_mouse_entered)
	area2d.mouse_exited.connect(_on_mouse_exited)
	area2d.input_event.connect(_on_input_event)
	self.set_state(PointState.CONTROL_HIDDEN)

func _on_mouse_entered():
	mouse_entered.emit()

func _on_mouse_exited():
	mouse_exited.emit()

func _on_input_event(_viewport: Viewport, event: InputEvent, _shape_idx: int):
	if event is InputEventMouseButton:
		if event.is_pressed() and event.button_index == MOUSE_BUTTON_LEFT:
			pressed.emit()
		elif event.is_released() and event.button_index == MOUSE_BUTTON_LEFT:
			released.emit()

func set_state(state : int):
	if current_state == state:
		return
	state_nodes[current_state].hide()
	current_state = state
	state_nodes[current_state].show()
	var new_radius = state_radii[current_state]
	if new_radius != 0:
		area2d.monitoring = true
		area2d.input_pickable = true
		($Area2D/CollisionShape2D.shape as CircleShape2D).radius = new_radius
	else:
		area2d.input_pickable = true
		area2d.monitoring = false


func update():
	self.set_state(controller.state)
	if controller.visual_info.focused:
		self.modulate = Color.from_rgba8(200,200,200)
	else:
		self.modulate = Color.WHITE
	if controller.visual_info.selected:
		self.scale = Vector2.ONE * 1.5
	else:
		self.scale = Vector2.ONE * 1.0
			
