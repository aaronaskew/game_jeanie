class_name Pung
extends Node2D

@onready var player_score_label: Label = %PlayerScore
@onready var ai_score_label: Label = %AIScore
@onready var player_paddle = %PlayerPaddle
@onready var ai_paddle = %AIPaddle


var ball: Ball = null
var pung_root: Pung

const PUNG_PLAY_AREA_SIZE: Vector2i = Vector2i(1920, 1248)
const PUNG_PLAY_AREA_START_Y: int = 160
const PUNG_PLAY_AREA: Rect2i = Rect2i(Vector2i(0, PUNG_PLAY_AREA_START_Y), PUNG_PLAY_AREA_SIZE)

var ball_scene = preload("res://scenes/pung/ball.tscn")

var player_score: int = 0
var ai_score: int = 0

func _ready():
	pung_root = find_parent("Pung")
	reset_ball(null)

func _process(dt: float) -> void:
	player_score_label.text = str(player_score)
	ai_score_label.text = str(ai_score)

	# Move player
	if Input.is_action_pressed("ui_up"):
		player_paddle.move_y(-1)

	if Input.is_action_pressed("ui_down"):
		player_paddle.move_y(1)


	# Move AI
	var direction: float = ball.position.y - ai_paddle.position.y
	if direction > ball.speed*dt:
		ai_paddle.move_y(1)
	elif direction < -ball.speed*dt:
		ai_paddle.move_y(-1)


func _on_ball_ai_scored(ball_node: RigidBody2D) -> void:
	ai_score += 1
	reset_ball(ball_node)


func _on_ball_player_scored(ball_node: RigidBody2D) -> void:
	player_score += 1
	reset_ball(ball_node)

func reset_ball(existing_ball: RigidBody2D):
	if existing_ball:
		existing_ball.queue_free()

	ball = ball_scene.instantiate()

	ball.player_scored.connect(_on_ball_player_scored)
	ball.ai_scored.connect(_on_ball_ai_scored)

	add_child(ball)
