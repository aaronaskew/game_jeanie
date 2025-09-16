class_name GamePlay
extends Node2D

const TV_POSITION: Vector2 = Vector2(1694, 240)

var pung_scene: PackedScene = preload("res://scenes/pung/Pung.tscn")
var beef_blastoids_scene: PackedScene = preload("res://scenes/beef_blastoids/BeefBlastoids.tscn")
var race_place_scene: PackedScene = preload("res://scenes/race_place/RacePlace.tscn")

var current_game = null

# @onready var viewport: SubViewport = $SubViewport


func choose_game(game: String):
	if current_game != null:
		current_game.queue_free()

	match game:
		"pung":
			current_game = pung_scene.instantiate()
		"beef_blastoids":
			current_game = beef_blastoids_scene.instantiate()
		"race_place":
			current_game = race_place_scene.instantiate()

	assert(current_game is Node2D)
	current_game.position = TV_POSITION
	add_child.call_deferred(current_game, true)
	# viewport.add_child.call_deferred(current_game, true)


func _input(event: InputEvent) -> void:
	if OS.is_debug_build():
		if event is InputEventKey && event.is_pressed():
			match event.keycode:
				KEY_P:
					choose_game("pung")
				KEY_B:
					choose_game("beef_blastoids")
				KEY_R:
					choose_game("race_place")
