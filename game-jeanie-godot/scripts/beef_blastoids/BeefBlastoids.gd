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

@onready var lives_label: Label = %Lives
@onready var score_label: Label = %Score
@onready var ui: Control = $UI
@onready var ship_scene = preload("res://scenes/beef_blastoids/ship.tscn")
@onready var beef_scene = preload("res://scenes/beef_blastoids/beef.tscn")


func _ready():
	canvas_size = ui.size

	spawn_ship()

	spawn_beef()


func _process(_dt):
	lives_label.text = str(lives)
	score_label.text = str(score)

	if lives == 0:
		game_over()


func _on_ship_death():
	lives -= 1
	if lives > 0:
		spawn_ship()
	print("ship done died")


func game_over():
	print("game_over")


func spawn_ship():
	var ship: Ship = ship_scene.instantiate()

	ship.canvas_size = canvas_size
	ship.position = canvas_size / 2.0

	ship.make_death_process.connect(_on_ship_death)

	add_child(ship)


func spawn_beef():
	for i in range(initial_num_beef):
		var beef: Beef = beef_scene.instantiate()

		beef.initialize(
			Beef.Size.LARGE,
			canvas_size,
			Vector2(randf_range(0, canvas_size.x), randf_range(0, canvas_size.y))
		)

		add_child(beef)
