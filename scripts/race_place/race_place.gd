class_name RacePlace
extends Node2D

@export var player_max_speed: Vector2 = Vector2(2000.0, 500.0)

@onready var animation_player: AnimationPlayer = $AnimationPlayer
@onready var player: Car = $Player


func _ready():
	animation_player.speed_scale = 0
	animation_player.play("road")


func _process(_delta):
	animation_player.speed_scale = -player.linear_velocity.y / player_max_speed.y
