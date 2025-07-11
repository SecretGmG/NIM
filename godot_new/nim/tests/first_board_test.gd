extends Node2D

@onready var board : BoardController = $BoardController

var activated_point = -1
var focused_point = -1


func _ready() -> void:
	board.point_clicked.connect(_on_point_clicked)
	board.line_clicked.connect(_on_line_clicked)
	board.point_entered.connect(_on_point_entered)
	board.line_entered.connect(_on_line_entered)
	board.point_exited.connect(_on_point_exited)
	board.line_exited.connect(_on_line_exited)
	
	var point_ids = []
	for pos in [Vector2(100,100), Vector2(300,300), Vector2(400,450), Vector2(450,300)]:
		var point_id = board.spawn_point()
		board.get_point(point_id).global_position = pos
		print('point {0} spawned'.format([point_id]))
		point_ids.append(point_id)
	
	
	var line_id = board.spawn_line()
	for id in point_ids:
		board.insert_point_to_line(id, line_id, -1)
	

func _on_point_entered(id):
	if focused_point != -1:
		board.set_point_focused(focused_point, false)
	focused_point = id
	board.set_point_focused(focused_point, false)
	
	print('point entered {0}'.format([id]))

func _on_line_entered(id):
	print('line entered {0}'.format([id]))

func _on_point_exited(id):
	if id == focused_point:
		board.set_point_focused(focused_point, false)
		focused_point = -1
	print('point exited {0}'.format([id]))

func _on_line_exited(id):
	print('line exited {0}'.format([id]))

func _on_point_clicked(id):
	print('point pressed {0}'.format([id]))

func _on_line_clicked(id):
	print('line pressed {0}'.format(id))
