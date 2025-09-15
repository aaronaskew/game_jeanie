class_name Ball
extends RigidBody2D

signal player_scored
signal ai_scored

@export var radius: float = 20.0
@export var segments: int = 32
@export var speed: float = 1000.0

var pung_root: Pung

@onready var ball: Polygon2D = $Polygon2D
@onready var collider: CollisionShape2D = $CollisionShape2D


func _ready():
	pung_root = find_parent("Pung")

	create_circle(radius, segments)
	ball.color = Color.WHITE

	var circle_shape: CircleShape2D = collider.shape
	circle_shape.radius = radius

	reset()


func _physics_process(_delta):
	if position.x > pung_root.PUNG_PLAY_AREA.end.x:
		player_scored.emit(self)
		reset()

	if position.x < pung_root.PUNG_PLAY_AREA.position.x:
		ai_scored.emit(self)
		reset()


func reset():
	var new_pos = Vector2(pung_root.PUNG_PLAY_AREA.end) / 2.0
	global_transform.origin = to_global(new_pos)
	linear_velocity = Vector2(1, -1).normalized() * speed


func create_circle(p_radius: float, p_segments: int):
	var points = PackedVector2Array()
	for i in range(p_segments):
		var angle = 2.0 * PI * i / p_segments
		var x = p_radius * cos(angle)
		var y = p_radius * sin(angle)
		points.append(Vector2(x, y))
	ball.polygon = points
