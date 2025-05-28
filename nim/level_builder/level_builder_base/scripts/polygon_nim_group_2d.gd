@tool
extends Polygon2D

func _ready():
	if Engine.is_editor_hint():
		var nim_group = get_parent()
		assert(nim_group is NimGroup, 'Parent of PolygonNimGroup needs to be a NimGroup')
		nim_group.get_parent().set_editable_instance(nim_group, true)
	else:
		fetch_members()

func fetch_members():
	var level_node = get_parent()
	while not level_node is NimLevel:
		level_node = level_node.get_parent()
		assert(level_node != null, 'PolygonNimGroup needs to be a child node of a NimLevel!')
	
	var nim_group = get_parent()
	assert(nim_group is NimGroup, 'Parent of PolygonNimGroup needs to be a NimGroup')
	
	for nim_node in level_node.get_nim_nodes():
		if Geometry2D.is_point_in_polygon(to_local(nim_node.global_position), polygon):
			nim_group.add_member(nim_node)
