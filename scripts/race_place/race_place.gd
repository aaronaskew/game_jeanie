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
@onready var ready_set_audio: AudioStreamPlayer = %ReadySet
@onready var go_audio: AudioStreamPlayer = %Go
@onready var finish_line: TileMapLayer = $FinishLine
@onready var state_chart: StateChart = $StateChart
@onready var intro_state: AtomicState = $StateChart/CompoundState/Intro
@onready var intro_ui: PanelContainer = $UI/IntroUI
@onready var game_over_ui: PanelContainer = $UI/GameOverUI
@onready var game_over_win_state: AtomicState = get_node("StateChart/CompoundState/GameOver/Win")
@onready var game_over_lose_state: AtomicState = get_node("StateChart/CompoundState/GameOver/Lose")
@onready var game_over_result: Label = game_over_ui.get_node(
	"MarginContainer/Panel/CenterContainer/VBoxContainer/GameOverResult"
)


func _ready():
	# Move the finish line to match the race length
	finish_line.position.y = player.position.y - length_of_race
	init_finish_line_position_y = finish_line.position.y

	fuel_gauge.value = 100.0


func _process(_delta):
	animation_player.speed_scale = -player.linear_velocity.y / ROADSIDE_ANIM_SPEED

	if player.current_fuel <= 0 && abs(player.linear_velocity.y) < 1.0:
		state_chart.send_event("game_lost")
		print("player lost")

	update_fuel_gauge()


func _physics_process(_delta):
	var new_finish_line_position_y = init_finish_line_position_y - player.virtual_y_pos
	finish_line.position.y = new_finish_line_position_y


func start_countdown():
	animation_player.speed_scale = 0
	animation_player.play("road")

	ready_go_label.text = "READY"
	ready_go_label.add_theme_color_override("font_color", Color.RED)

	ready_set_audio.play()


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
		state_chart.send_event("game_won")
		print("player won")


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
	start_countdown()


func _on_game_over_state_entered() -> void:
	game_over_ui.visible = true


func _on_win_state_entered() -> void:
	game_over_result.text = "You win!"


func _on_lose_state_entered() -> void:
	game_over_result.text = "You lose"
