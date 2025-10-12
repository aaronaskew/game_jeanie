class_name BeefBlastoids
extends Node2D

# pub DESCRIPTION: String,
# pub MAX_SCORE: u32,
# pub NUM_LIVES: u32,
# pub SHIP_THRUST_MAGNITUDE: f32,
# pub SHIP_MAX_VELOCITY: f32,
# pub SHIP_ROTATION_SPEED: f32,
# pub SHIP_INVINCIBLE_TIME: f32,
# pub SHIP_BLINK_RATE: f32,
# pub BLASTER_COOLDOWN: f32,
# pub BULLET_TTL: f32,
# pub BULLETS_DESPAWN: bool,
# pub BULLET_RADIUS: f32,
# pub BULLET_SPEED: f32,
# pub INITIAL_NUM_BEEF: u32,
# pub INITIAL_BEEF_SPEED: f32,
# pub BEEF_NUM_VERTS: u8,
# pub BEEF_RADIUS: f32,
# pub BEEF_RADIUS_VARIANCE: f32,
# pub BEEF_SCORE_VALUE: u32,

var canvas_size: Vector2
var lives: int = 3
var score: int = 0
var initial_num_beef: int = 2
var explosion_energy: float = 1.5

@onready var lives_label: Label = %Lives
@onready var score_label: Label = %Score
@onready var ui: Control = $UI
@onready var ship_scene = preload("res://scenes/beef_blastoids/ship.tscn")
@onready var beef_scene = preload("res://scenes/beef_blastoids/beef.tscn")
@onready var beef_explosion_scene = preload("res://scenes/beef_blastoids/beef_explosion.tscn")

@onready var state_chart: StateChart = $StateChart
@onready var intro_state: AtomicState = $StateChart/CompoundState/Intro
@onready var playing_state: AtomicState = $StateChart/CompoundState/Playing

@onready var intro_ui: PanelContainer = $UI/IntroUI
@onready var game_over_ui: PanelContainer = $UI/GameOverUI
@onready var game_over_result: Label = game_over_ui.get_node(
	"MarginContainer/Panel/CenterContainer/VBoxContainer/GameOverResult"
)


func _ready():
	canvas_size = ui.size


func _process(_dt):
	lives_label.text = str(lives)
	score_label.text = str(score)


func _on_ship_death(ship_explosion: GPUParticles2D):
	ship_explosion.finished.connect(_on_ship_explosion_finished)
	lives -= 1
	check_if_game_over()


func check_if_game_over():
	if score >= 10000:
		state_chart.send_event("game_won")
	elif lives <= 0:
		state_chart.send_event("game_lost")


func _on_ship_explosion_finished():
	if lives > 0:
		spawn_ship()


func spawn_ship():
	var ship: Ship = ship_scene.instantiate()

	ship.canvas_size = canvas_size
	ship.position = canvas_size / 2.0

	ship.make_death_process.connect(_on_ship_death)

	add_child(ship, true)


func spawn_initial_beef():
	for i in range(initial_num_beef):
		var beef: Beef = beef_scene.instantiate()

		beef.initialize(
			Beef.Size.LARGE,
			canvas_size,
			Vector2(randf_range(0, canvas_size.x), randf_range(0, canvas_size.y))
		)

		beef.set_random_velocities()

		add_child(beef, true)


func spawn_sharded_beef(
	size: Beef.Size, p_position: Vector2, p_linear_velocity: Vector2, p_angular_velocity: float
):
	var linear_velocity1 = p_linear_velocity.rotated(PI / 2) * explosion_energy
	var linear_velocity2 = p_linear_velocity.rotated(-PI / 2) * explosion_energy

	var beef1: Beef = beef_scene.instantiate()
	beef1.initialize(size, canvas_size, p_position)
	beef1.linear_velocity = linear_velocity1
	beef1.angular_velocity = p_angular_velocity * explosion_energy

	var beef2: Beef = beef_scene.instantiate()
	beef2.initialize(size, canvas_size, p_position)
	beef2.linear_velocity = linear_velocity2
	beef2.angular_velocity = -p_angular_velocity * explosion_energy

	add_child(beef1, true)
	add_child(beef2, true)


func _on_destroy_beef(node: Node2D):
	if node is Beef:
		var beef: Beef = node

		var beef_explosion: GPUParticles2D = beef_explosion_scene.instantiate()
		beef_explosion.position = beef.position
		add_child(beef_explosion, true)
		beef_explosion.emitting = true

		score += beef.score_value

		check_if_game_over()

		var old_position = beef.position
		var old_linear_velocity = beef.linear_velocity
		var old_angular_velocity = beef.angular_velocity

		beef.queue_free()

		match beef.size:
			beef.Size.LARGE:
				spawn_sharded_beef(
					Beef.Size.MEDIUM, old_position, old_linear_velocity, old_angular_velocity
				)
			beef.Size.MEDIUM:
				spawn_sharded_beef(
					Beef.Size.SMALL, old_position, old_linear_velocity, old_angular_velocity
				)
			beef.Size.SMALL:
				pass


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
	spawn_ship()
	spawn_initial_beef()


func _on_game_over_child_state_entered() -> void:
	game_over_ui.visible = true


func _on_win_state_entered() -> void:
	game_over_result.text = "You win!"


func _on_lose_state_entered() -> void:
	game_over_result.text = "You lose"
