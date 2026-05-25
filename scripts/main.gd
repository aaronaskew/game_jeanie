extends Node2D

@onready var game_manager: GameManager = $"/root/GameManagerScene"


func _ready():
	# Initialize GameManager active_scene variable to Main
	game_manager.active_scene = self

	# Immediately load the TitleScreen scene
	game_manager.load_scene(game_manager.title_screen_scene)
