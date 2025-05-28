extends Node
class_name NimGroup

@export var members : Array[NimNode] = []

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	
	pass # Replace with function body.

func add_member(nim_node: NimNode):
	members.append(nim_node)
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
