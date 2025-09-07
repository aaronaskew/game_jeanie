class_name BeefBlastoids
extends Node2D

@onready var lives_label: Label = %Lives
@onready var score_label: Label = %Score
@onready var ui: Control = $UI
@onready var ship: Ship = $Ship

var canvas_size: Vector2
var lives: int = 3
var score: int = 0

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


func _ready():
	canvas_size = ui.size
	ship.canvas_size = canvas_size
	ship.position = canvas_size / 2.0

func _process(_dt):
	lives_label.text = str(lives)
	score_label.text = str(score)
