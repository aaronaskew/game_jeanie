extends Node2D

@onready var state_chart_debugger = $StateChartDebugger
@onready var game_manager = $"/root/GameManager"


func _ready():
	state_chart_debugger.debug_node(game_manager.game_state)


func _input(event: InputEvent) -> void:
	if event is InputEventKey && event.is_pressed():
		match event.keycode:
			KEY_C:
				state_chart_debugger.visible = !state_chart_debugger.visible
