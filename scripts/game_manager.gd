class_name GameManager
extends Node

var game_wins: Dictionary = {
	"pung": 0,
	"beef_blastoids": 0,
	"race_place": 0,
}

# TODO: Refactor scene logic into a SceneManager global

# var game_state_scene: Resource = preload("res://scenes/GameState.tscn")
var debug_scene: Resource = preload("res://scenes/Debug.tscn")
var gameplay_scene: Resource = preload("res://scenes/GamePlay.tscn")
var title_screen_scene: Resource = preload("res://scenes/TitleScreen.tscn")
var cut_scenes_scene: Resource = preload("res://scenes/CutScenes.tscn")
var game_select_screen_scene: Resource = preload("res://scenes/GameSelect.tscn")

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


func game_select_screen():
	game_state.send_event("game_select")


func load_scene(scene: Resource):
	if active_scene:
		active_scene.queue_free()

	active_scene = scene.instantiate()
	add_sibling.call_deferred(active_scene)


func _on_title_screen_state_entered() -> void:
	load_scene(title_screen_scene)


func _on_cut_scene_start_state_entered() -> void:
	load_scene(cut_scenes_scene)

	# TODO: Add logic to choose which dialogue block to start at


func _on_game_select_state_entered() -> void:
	print("loading game select screen")
	load_scene(game_select_screen_scene)
