extends Node2D

class_name PointView

signal mouse_entered
signal mouse_exited
signal clicked

@onready var controller : PointController = $".."

@onready var sprites : Dictionary [int, Sprite2D] = {
	PointState.DEFAULT: $SpriteDefault,
	PointState.ACTIVATED: $SpriteActive,
	PointState.CONTROL_HIDDEN: $SpriteHidden,
	PointState.CONTROL_VISIBLE: $SpriteControl,
}

@onready var area: Area2D = $Area2D

var current_state : int = PointState.DEFAULT


func _ready():
	area.mouse_entered.connect(_on_mouse_entered)
	area.mouse_exited.connect(_on_mouse_exited)
	area.input_event.connect(_on_input_event)

func _on_mouse_entered():
	mouse_entered.emit()

func _on_mouse_exited():
	mouse_exited.emit()

func _on_input_event(_viewport: Viewport, event: InputEvent, _shape_idx: int):
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		clicked.emit()

func set_state(state : int):
	if current_state == state:
		return
	sprites[current_state].hide()
	current_state = state
	sprites[current_state].show()


func update():
	self.set_state(controller.state)
	var sprite = self.sprites[current_state]
	if controller.visual_info.focused:
		self.modulate = Color.from_rgba8(200,200,200)
	else:
		self.modulate = Color.WHITE
	if controller.visual_info.selected:
		self.scale = Vector2.ONE * 1.5
	else:
		self.scale = Vector2.ONE * 1.0
			
