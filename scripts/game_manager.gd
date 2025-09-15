class_name GameManager
extends Node

var game_wins = {
	"pung": 0,
	"beef_blastoids": 0,
	"race_place": 0,
}

var game_state: StateChart

@onready var game_state_scene = preload("res://scenes/GameState.tscn")
@onready var debug_scene = preload("res://scenes/Debug.tscn")


func _ready():
	game_state = game_state_scene.instantiate()
	game_state.set_expression_property.call_deferred("pung_wins", game_wins["pung"])
	game_state.set_expression_property.call_deferred(
		"beef_blastoids_wins", game_wins["beef_blastoids"]
	)
	game_state.set_expression_property.call_deferred("race_place_wins", game_wins["race_place"])
	add_child(game_state)

	if OS.is_debug_build():
		add_child(debug_scene.instantiate())


func log_game_win(game: String):
	game_wins[game] += 1
	game_state.set_expression_property(game + "_wins", game_wins[game])
