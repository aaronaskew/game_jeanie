class_name GameSelect
extends Node2D

@onready var game_manager: GameManager = $"/root/GameManagerScene"
@onready var game_state: StateChart = $"/root/GameManagerScene/GameState"

@onready var pung_glow: Sprite2D = $Panel2PungGlow
@onready var beef_blastoids_glow: Sprite2D = $Panel2BlastoidGlow
@onready var race_place_glow: Sprite2D = $Panel2RaceplaceGlow

@onready var pung_seal: Sprite2D = $Panel2PungSeal
@onready var beef_blastoids_seal: Sprite2D = $Panel2BlastoidSeal
@onready var race_place_seal: Sprite2D = $Panel2RaceplaceSeal


# TODO: change in statechart instead
func _ready():
	if game_manager.game_wins["pung"] > 0:
		pung_seal.visible = true
	if game_manager.game_wins["beef_blastoids"] > 0:
		beef_blastoids_seal.visible = true
	if game_manager.game_wins["race_place"] > 0:
		race_place_seal.visible = true


func _on_pung_area_2d_mouse_entered() -> void:
	pung_glow.visible = true


func _on_pung_area_2d_mouse_exited() -> void:
	pung_glow.visible = false


func _on_beef_blastoids_area_2d_mouse_entered() -> void:
	beef_blastoids_glow.visible = true


func _on_beef_blastoids_area_2d_mouse_exited() -> void:
	beef_blastoids_glow.visible = false


func _on_race_place_area_2d_mouse_entered() -> void:
	race_place_glow.visible = true


func _on_race_place_area_2d_mouse_exited() -> void:
	race_place_glow.visible = false


func _on_pung_area_2d_input_event(_viewport: Node, event: InputEvent, _shape_idx: int) -> void:
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT && event.pressed:
			# game_manager.play_game("pung")
			game_state.send_event("play_pung")
			queue_free()


func _on_beef_blastoids_area_2d_input_event(
	_viewport: Node, event: InputEvent, _shape_idx: int
) -> void:
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT && event.pressed:
			# game_manager.play_game("beef_blastoids")
			game_state.send_event("play_beef_blastoids")
			queue_free()


func _on_race_place_area_2d_input_event(
	_viewport: Node, event: InputEvent, _shape_idx: int
) -> void:
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT && event.pressed:
			# game_manager.play_game("race_place")
			game_state.send_event("play_race_place")
			queue_free()
