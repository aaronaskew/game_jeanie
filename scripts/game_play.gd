class_name GamePlay
extends Node2D

const TV_POSITION: Vector2 = Vector2(1694, 240)

var pung_scene = preload("res://scenes/pung/Pung.tscn")
var beef_blastoids_scene = preload("res://scenes/beef_blastoids/BeefBlastoids.tscn")
var race_place_scene = preload("res://scenes/race_place/RacePlace.tscn")

var current_game


func choose_game(game: String):
	match game:
		"pung":
			current_game = pung_scene.instantiate()
		"beef_blastoids":
			current_game = beef_blastoids_scene.instantiate()
		"race_place":
			current_game = race_place_scene.instantiate()

	current_game.position = TV_POSITION

	add_child(current_game)
