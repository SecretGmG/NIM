extends Node2D
class_name  NimLevel


var children_are_fetched = false
var nim_nodes: Array[NimNode] = []
var nim_groupes: Array[NimGroup] = []
var grouped_nodes: Dictionary

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	debug_print_level_data()
	pass # Replace with functionbody.

func get_nim_groupes():
	if not children_are_fetched:
		fetch_children()
		children_are_fetched = true
	return self.nim_groupes
func get_nim_nodes():
	if not children_are_fetched:
		fetch_children()
		children_are_fetched = true
	return self.nim_nodes

func fetch_children():
	for child in get_children(true):
		if child is NimGroup:
			self.nim_groupes.append(child)
		if child is NimNode:
			self.nim_nodes.append(child)

func debug_print_level_data():
	print('nodes:')
	print(', '.join(self.nim_nodes.map(func(x): return x.name)))
	print('groups nodes:')
	for group in self.nim_groupes:
		print(group.name)
		print(', '.join(group.members.map(func(x): return x.name)))

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
