class_name EngineAudio
extends AudioStreamPlayer

@export var min_pitch_scale: float = 0.1
@export var max_pitch_scale: float = 0.5

@onready var player: PlayerCar = %PlayerCar


func _process(_dt):
	pitch_scale = (
		-player.linear_velocity.y / player.max_speed.y * (max_pitch_scale - min_pitch_scale)
		+ min_pitch_scale
	)
