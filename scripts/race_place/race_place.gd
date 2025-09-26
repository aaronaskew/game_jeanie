class_name RacePlace
extends Node2D

signal start_race

const ROADSIDE_ANIM_SPEED: float = 320.0

@export var max_roadside_speed_scale: float = 10.0
@export var length_of_race: float = 10_000.0

var init_finish_line_position_y: float

@onready var animation_player: AnimationPlayer = $AnimationPlayer
@onready var player: PlayerCar = $PlayerCar
@onready var fuel_gauge: ProgressBar = %FuelGauge
@onready var ready_go_label: Label = %ReadyGoLabel
@onready var go_audio: AudioStreamPlayer = %Go
@onready var finish_line: TileMapLayer = $FinishLine


func _ready():
	animation_player.speed_scale = 0
	animation_player.play("road")

	fuel_gauge.value = 100.0

	ready_go_label.text = "READY"
	ready_go_label.add_theme_color_override("font_color", Color.RED)

	# Move the finish line to match the race length
	finish_line.position.y = player.position.y - length_of_race

	init_finish_line_position_y = finish_line.position.y


func _process(_delta):
	# animation_player.speed_scale = -player.linear_velocity.y / player_max_speed.y
	animation_player.speed_scale = -player.linear_velocity.y / ROADSIDE_ANIM_SPEED

	update_fuel_gauge()


func _physics_process(_delta):
	finish_line.position.y = floor(init_finish_line_position_y - player.virtual_y_pos)


func update_fuel_gauge():
	fuel_gauge.value = player.current_fuel / player.max_fuel * 100.0


func _on_ready_set_finished() -> void:
	start_race.emit()
	go_audio.play()
	ready_go_label.text = "GO!"
	ready_go_label.add_theme_color_override("font_color", Color.GREEN)
	await get_tree().create_timer(1.0).timeout
	ready_go_label.visible = false


func _on_finish_line_body_entered(body: Node2D) -> void:
	if body is PlayerCar:
		print("Player Wins!")
	elif body is EnemyCar:
		print("Player Loses!")
