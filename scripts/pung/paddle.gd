extends StaticBody2D

@export var speed: float = 500.0
@export var size = Vector2(16, 128)

var pung_root: Pung
var move_direction = 0

@onready var collision_shape: CollisionShape2D = $CollisionShape2D
@onready var polygon: Polygon2D = $Polygon2D


func _ready() -> void:
	set_size(size)

	pung_root = find_parent("Pung")


func _physics_process(dt):
	#set_size(size)
	if move_direction != 0:
		position.y += move_direction * speed * dt
		move_direction = 0

	position.y = clampf(
		position.y,
		pung_root.PUNG_PLAY_AREA.position.y + size.y / 2,
		pung_root.PUNG_PLAY_AREA.end.y - size.y / 2
	)


func set_size(new_size: Vector2):
	var collision_rect: RectangleShape2D = collision_shape.shape
	collision_rect.size = new_size

	var paddle_polygon: PackedVector2Array

	paddle_polygon.push_back(Vector2(-new_size.x / 2, -new_size.y / 2))
	paddle_polygon.push_back(Vector2(new_size.x / 2, -new_size.y / 2))
	paddle_polygon.push_back(Vector2(new_size.x / 2, new_size.y / 2))
	paddle_polygon.push_back(Vector2(-new_size.x / 2, new_size.y / 2))

	polygon.polygon = paddle_polygon


func move_y(direction: int):
	move_direction = direction
