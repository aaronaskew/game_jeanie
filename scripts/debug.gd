extends Node2D

@onready var state_chart_debugger = $StateChartDebugger
@onready var game_manager: GameManager = $"/root/GameManagerScene"


func _ready():
	state_chart_debugger.debug_node(game_manager.game_state)


func _input(event: InputEvent) -> void:
	if event is InputEventKey && event.is_pressed():
		match event.keycode:
			KEY_C:
				state_chart_debugger.visible = !state_chart_debugger.visible
			KEY_1:
				game_manager.log_game_win("pung")
			KEY_2:
				game_manager.log_game_win("beef_blastoids")
			KEY_3:
				game_manager.log_game_win("race_place")
			KEY_4:
				game_manager.game_state.send_event("genie_arrived")
			KEY_5:
				game_manager.game_state.send_event("activate_game_jeanie")
