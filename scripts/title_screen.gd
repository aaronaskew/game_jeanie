extends Control

@onready var game_manager: GameManager = $"/root/GameManagerScene"


func _on_start_game_button_pressed() -> void:
	game_manager.start_game()
