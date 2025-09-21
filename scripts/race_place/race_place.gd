class_name RacePlace
extends Node2D

@export var max_roadside_speed_scale: float = 10.0

@onready var animation_player: AnimationPlayer = $AnimationPlayer
@onready var player: Car = $Player


func _ready():
	animation_player.speed_scale = 0
	animation_player.play("road")


func _process(_delta):
	# animation_player.speed_scale = -player.linear_velocity.y / player_max_speed.y
	animation_player.speed_scale = (
		-player.linear_velocity.y / player.max_speed.y * max_roadside_speed_scale
	)
