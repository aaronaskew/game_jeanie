class_name GameManager
extends Node

var game_wins = {
	"pung": 0,
	"beef_blastoids": 0,
	"race_place": 0,
}

# var game_state_scene = preload("res://scenes/GameState.tscn")
var debug_scene = preload("res://scenes/Debug.tscn")
var gameplay_scene = preload("res://scenes/GamePlay.tscn")
var title_screen_scene = preload("res://scenes/TitleScreen.tscn")

var active_scene: Node

@onready var game_state: StateChart = $GameState


func _ready():
	game_state.set_expression_property("pung_wins", game_wins["pung"])
	game_state.set_expression_property("beef_blastoids_wins", game_wins["beef_blastoids"])
	game_state.set_expression_property("race_place_wins", game_wins["race_place"])

	if OS.is_debug_build():
		add_child(debug_scene.instantiate())


func log_game_win(game: String):
	game_wins[game] += 1
	game_state.set_expression_property(game + "_wins", game_wins[game])


func play_game(game: String):
	var gameplay: GamePlay = gameplay_scene.instantiate()
	gameplay.choose_game(game)
	add_sibling(gameplay)


func start_game():
	game_state.send_event("start_game")


func _on_title_screen_state_entered() -> void:
	var title_screen_menu_node = get_node_or_null("/root/TitleScreenMenu")

	if title_screen_menu_node == null:
		active_scene = title_screen_scene.instantiate()
		add_sibling(active_scene)
	else:
		active_scene = title_screen_menu_node


func _on_title_screen_state_exited() -> void:
	if active_scene:
		active_scene.queue_free()
