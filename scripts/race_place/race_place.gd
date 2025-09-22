class_name RacePlace
extends Node2D

@export var max_roadside_speed_scale: float = 10.0

@onready var animation_player: AnimationPlayer = $AnimationPlayer
@onready var player: Car = $Player
@onready var fuel_gauge: ProgressBar = %FuelGauge


func _ready():
	animation_player.speed_scale = 0
	animation_player.play("road")

	fuel_gauge.value = 100.0


func _process(_delta):
	# animation_player.speed_scale = -player.linear_velocity.y / player_max_speed.y
	animation_player.speed_scale = (
		-player.linear_velocity.y / player.max_speed.y * max_roadside_speed_scale
	)

	update_fuel_gauge()


func update_fuel_gauge():
	fuel_gauge.value = 100.0 - player.distance_traveled * player.fuel_effeciency / player.max_fuel
