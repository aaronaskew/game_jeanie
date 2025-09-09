class_name BlasterBullet
extends Area2D

signal destroy(node: Node2D)

@export var radius: float = 10.0
@export var segments: int = 32
@export var speed: float = 2000.0
@export var lifetime: float = 0.5

var velocity: Vector2

@onready var polygon: Polygon2D = $Polygon2D
@onready var collider: CollisionPolygon2D = $CollisionPolygon2D
@onready var timer: Timer = $Timer
@onready var beef_blastoids: BeefBlastoids = find_parent("BeefBlastoids")


func _ready():
	create_circle(radius, segments)
	body_entered.connect(_on_body_entered)
	destroy.connect(beef_blastoids._destroy_beef)
	timer.wait_time = lifetime
	timer.timeout.connect(_on_timeout)
	timer.start()


func _on_timeout():
	queue_free()


func _physics_process(dt: float) -> void:
	global_position += velocity * dt


func _on_body_entered(body: Node2D):
	destroy.emit(body)
	queue_free()


func initialize(p_position: Vector2, p_rotation: float):
	position = p_position
	velocity = Vector2.UP.rotated(p_rotation) * speed


func create_circle(p_radius: float, p_segments: int):
	var points = PackedVector2Array()
	for i in range(p_segments):
		var angle = 2.0 * PI * i / p_segments
		var x = p_radius * cos(angle)
		var y = p_radius * sin(angle)
		points.append(Vector2(x, y))
	polygon.polygon = points
	collider.polygon = points
