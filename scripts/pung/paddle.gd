extends StaticBody2D

@onready var collision_shape: CollisionShape2D = $CollisionShape2D
@onready var polygon: Polygon2D = $Polygon2D
var pung_root: Pung

@export var size = Vector2(16, 128)
@export var speed: float = 500.0

var move_direction = 0

func _ready() -> void:
	set_size(size)

	pung_root = find_parent("Pung")


func _physics_process(dt):
	#set_size(size)
	if move_direction != 0:
		position.y += move_direction * speed * dt
		move_direction = 0

	position.y = clampf(position.y, pung_root.PUNG_PLAY_AREA.position.y + size.y / 2, pung_root.PUNG_PLAY_AREA.end.y - size.y / 2)

func set_size(new_size: Vector2):
	var collision_rect: RectangleShape2D = collision_shape.shape
	collision_rect.size = new_size

	polygon.polygon[0] = Vector2(-new_size.x / 2, -new_size.y / 2)
	polygon.polygon[1] = Vector2(new_size.x / 2, -new_size.y / 2)
	polygon.polygon[2] = Vector2(new_size.x / 2, new_size.y / 2)
	polygon.polygon[3] = Vector2(-new_size.x / 2, new_size.y / 2)


func move_y(direction: int):
	move_direction = direction
