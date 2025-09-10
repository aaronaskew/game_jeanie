class_name Beef
extends StaticBody2D

enum Size { LARGE, MEDIUM, SMALL }

@export var num_verts: int = 10
@export var radius_variance: float = 0.25
@export var score_value: int = 100
@export var init_linear_speed: float = 100
@export var init_linear_speed_variance: float = 0.25
@export var init_angular_velocity: float = 1.0
@export var init_angular_velocity_variance: float = 0.25
@export var radius_small: float = 25.0
@export var radius_medium: float = 75.0
@export var radius_large: float = 150.0

var linear_velocity: Vector2
var angular_velocity: float
var canvas_size: Vector2
var size: Size
var radius: float

@onready var polygon: Polygon2D = $Polygon2D
@onready var collider: CollisionPolygon2D = $CollisionPolygon2D
@onready var wireframe_line: Line2D


func initialize(p_size: Size, p_canvas_size: Vector2, p_position: Vector2):
	size = p_size

	match size:
		Size.SMALL:
			radius = radius_small
		Size.MEDIUM:
			radius = radius_medium
		Size.LARGE:
			radius = radius_large

	canvas_size = p_canvas_size
	position = p_position


func set_random_velocities():
	linear_velocity = (
		Vector2(
			randf_range(-1 + init_linear_speed_variance, 1 - init_linear_speed_variance),
			randf_range(-1 + init_linear_speed_variance, 1 - init_linear_speed_variance)
		)
		* init_linear_speed
	)

	angular_velocity = randf_range(
		init_angular_velocity * (-1 + init_angular_velocity_variance),
		init_angular_velocity * (1 - init_angular_velocity_variance)
	)


func _ready():
	var polygon_data = generate_beef_polygon()
	polygon.set_deferred("polygon", polygon_data)
	collider.set_deferred("polygon", polygon_data)

	call_deferred("create_wireframe")


func _physics_process(dt):
	position += linear_velocity * dt
	rotation += angular_velocity * dt

	if position.x > canvas_size.x:
		position.x = position.x - canvas_size.x
	elif position.x < 0:
		position.x = canvas_size.x - position.x

	if position.y > canvas_size.y:
		position.y = position.y - canvas_size.y
	elif position.y < 0:
		position.y = canvas_size.y - position.y


func generate_beef_polygon() -> PackedVector2Array:
	var points = PackedVector2Array()
	for i in range(num_verts):
		var angle = 2.0 * PI * i / num_verts
		var x = radius * (1 + randf_range(-radius_variance, radius_variance)) * cos(angle)
		var y = radius * (1 + randf_range(-radius_variance, radius_variance)) * sin(angle)
		points.append(Vector2(x, y))

	return points


func create_wireframe():
	# Create a Line2D node for the wireframe
	wireframe_line = Line2D.new()
	add_child(wireframe_line)

	# Copy polygon points to line
	wireframe_line.clear_points()
	for point in polygon.polygon:
		wireframe_line.add_point(point)

	# Close the polygon by adding the first point again
	if polygon.polygon.size() > 0:
		wireframe_line.add_point(polygon.polygon[0])

	# Style the wireframe
	wireframe_line.width = 5.0
	wireframe_line.default_color = Color.WHITE
	wireframe_line.joint_mode = Line2D.LINE_JOINT_SHARP

	# Hide the filled polygon
	polygon.visible = false
