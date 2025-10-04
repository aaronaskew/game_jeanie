extends Node2D

var current_state_chart_index = 0

@onready var state_chart_debugger: MarginContainer = $CanvasLayer/StateChartDebugger
@onready var game_manager: GameManager = $"/root/GameManagerScene"


func _ready():
	state_chart_debugger.visible = false


func _input(event: InputEvent) -> void:
	if event is InputEventKey && event.is_pressed():
		match event.keycode:
			KEY_C:
				var state_charts: Array[StateChart]
				state_charts.append(game_manager.game_state)
				var race_place_state: StateChart = get_node_or_null(
					"/root/GamePlay/RacePlace/StateChart"
				)
				if race_place_state != null:
					state_charts.append(race_place_state)

				if !state_chart_debugger.visible:
					current_state_chart_index = 0
					state_chart_debugger.debug_node(state_charts[current_state_chart_index])
					state_chart_debugger.visible = true
				else:
					current_state_chart_index = (
						(current_state_chart_index + 1) % state_charts.size()
					)

					if current_state_chart_index == 0:
						state_chart_debugger.visible = false
					else:
						state_chart_debugger.debug_node(state_charts[current_state_chart_index])

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
