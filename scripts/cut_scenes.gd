class_name CutScenes
extends Node2D

var panels: Array[Node]
var current_panel_idx

var cut_scenes_dialogue: Resource = load("res://assets/dialogue/cut_scenes.dialogue")

@onready var animation_player: AnimationPlayer = $AnimationPlayer


func _ready():
	animation_player.animation_finished.connect(_on_animation_finished)
	print("showing dialogue balloon")

	# Ensure that DialogueManager is using CutScenes as the current scene
	DialogueManager.get_current_scene = func(): return self

	DialogueManager.show_dialogue_balloon(cut_scenes_dialogue, "StartA")
	print("launched dialogue balloon")


func _process(_delta: float) -> void:
	pass


func _on_animation_finished(_animation_name: String):
	pass


func play_animation(p_animation: String):
	animation_player.play(p_animation)
	await animation_player.animation_finished


func cutscene_print(msg: String):
	print(msg)
