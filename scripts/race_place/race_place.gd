class_name RacePlace
extends Node2D

signal start_race

@export var max_roadside_speed_scale: float = 10.0

@onready var animation_player: AnimationPlayer = $AnimationPlayer
@onready var player: PlayerCar = $PlayerCar
@onready var fuel_gauge: ProgressBar = %FuelGauge
@onready var ready_go_label: Label = %ReadyGoLabel
@onready var go_audio: AudioStreamPlayer = %Go


func _ready():
	animation_player.speed_scale = 0
	animation_player.play("road")

	fuel_gauge.value = 100.0

	ready_go_label.text = "READY"
	ready_go_label.add_theme_color_override("font_color", Color.RED)


func _process(_delta):
	# animation_player.speed_scale = -player.linear_velocity.y / player_max_speed.y
	animation_player.speed_scale = (
		-player.linear_velocity.y / player.max_speed.y * max_roadside_speed_scale
	)

	update_fuel_gauge()


func update_fuel_gauge():
	fuel_gauge.value = 100.0 - player.distance_traveled * player.fuel_effeciency / player.max_fuel


func _on_ready_set_finished() -> void:
	start_race.emit()
	go_audio.play()
	ready_go_label.text = "GO!"
	ready_go_label.add_theme_color_override("font_color", Color.GREEN)
	await get_tree().create_timer(1.0).timeout
	ready_go_label.visible = false
