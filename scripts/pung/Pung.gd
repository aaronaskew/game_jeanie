class_name Pung
extends Node2D

const PUNG_PLAY_AREA_SIZE: Vector2i = Vector2i(1920, 1248)
const PUNG_PLAY_AREA_START_Y: int = 160
const PUNG_PLAY_AREA: Rect2i = Rect2i(Vector2i(0, PUNG_PLAY_AREA_START_Y), PUNG_PLAY_AREA_SIZE)

var ball: Ball = null
var pung_root: Pung

var ball_scene = preload("res://scenes/pung/ball.tscn")

var player_score: int = 0
var ai_score: int = 9

@onready var player_score_label: Label = %PlayerScore
@onready var ai_score_label: Label = %AIScore
@onready var player_paddle = %PlayerPaddle
@onready var ai_paddle = %AIPaddle

@onready var state_chart: StateChart = $StateChart
@onready var intro_state: AtomicState = $StateChart/CompoundState/Intro
@onready var playing_state: AtomicState = $StateChart/CompoundState/Playing

@onready var intro_ui: PanelContainer = $UI/IntroUI
@onready var game_over_ui: PanelContainer = $UI/GameOverUI
@onready var game_over_result: Label = game_over_ui.get_node(
	"MarginContainer/Panel/CenterContainer/VBoxContainer/GameOverResult"
)


func _ready():
	pung_root = find_parent("Pung")


func _process(dt: float) -> void:
	if playing_state.active:
		player_score_label.text = str(player_score)
		ai_score_label.text = str(ai_score)

		# Move player
		if Input.is_action_pressed("ui_up"):
			player_paddle.move_y(-1)

		if Input.is_action_pressed("ui_down"):
			player_paddle.move_y(1)

		# Move AI
		var direction: float = ball.position.y - ai_paddle.position.y
		if direction > ball.speed * dt:
			ai_paddle.move_y(1)
		elif direction < -ball.speed * dt:
			ai_paddle.move_y(-1)


func _on_ball_ai_scored(ball_node: RigidBody2D) -> void:
	ai_score += 1
	check_if_game_over()
	if playing_state.active:
		reset_ball(ball_node)


func _on_ball_player_scored(ball_node: RigidBody2D) -> void:
	player_score += 1
	check_if_game_over()
	if playing_state.active:
		reset_ball(ball_node)


func check_if_game_over():
	if player_score >= 10 || ai_score >= 10:
		ball.queue_free()

	if player_score >= 10:
		state_chart.send_event("game_won")
	elif ai_score >= 10:
		state_chart.send_event("game_lost")


func reset_ball(existing_ball: RigidBody2D):
	if existing_ball:
		existing_ball.queue_free()

	ball = ball_scene.instantiate()

	ball.player_scored.connect(_on_ball_player_scored)
	ball.ai_scored.connect(_on_ball_ai_scored)

	add_child(ball, true)


func _unhandled_input(event: InputEvent) -> void:
	if intro_state.active:
		if event.is_action_pressed("ui_accept"):
			intro_ui.visible = false
			state_chart.send_event("intro_finished")


func _on_intro_state_entered() -> void:
	intro_ui.visible = true


func _on_intro_state_exited() -> void:
	intro_ui.visible = false


func _on_playing_state_entered() -> void:
	reset_ball(null)


func _on_game_over_child_state_entered() -> void:
	game_over_ui.visible = true


func _on_win_state_entered() -> void:
	game_over_result.text = "You win!"


func _on_lose_state_entered() -> void:
	game_over_result.text = "You lose"
