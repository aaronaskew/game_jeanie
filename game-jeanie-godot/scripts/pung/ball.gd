extends RigidBody2D

@onready var ball: Polygon2D = $Polygon2D
@onready var collider: CollisionShape2D = $CollisionShape2D

@export var radius: float = 20.0
@export var segments: int = 32
@export var speed: float = 1000.0

func _ready():
	create_circle(radius, segments)
	ball.color = Color.WHITE

	var circle_shape: CircleShape2D = collider.shape
	circle_shape.radius = radius

	linear_velocity = Vector2(1.0, -1.0).normalized() * speed


func create_circle(p_radius: float, p_segments: int):
	var points = PackedVector2Array()
	for i in range(p_segments):
		var angle = 2.0 * PI * i / p_segments
		var x = p_radius * cos(angle)
		var y = p_radius * sin(angle)
		points.append(Vector2(x, y))
	ball.polygon = points
