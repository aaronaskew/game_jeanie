class_name Ship
extends RigidBody2D

# pub SHIP_THRUST_MAGNITUDE: f32,
# pub SHIP_MAX_VELOCITY: f32,
# pub SHIP_ROTATION_SPEED: f32,
# pub SHIP_INVINCIBLE_TIME: f32,
# pub SHIP_BLINK_RATE: f32,

@export var thrust_magnitude: float = 500.0
@export var max_speed: float = 1000.0
@export var rotation_speed: float = 10.0
@export var invincible_time: float = 3.0
@export var blink_rate: float = 0.5

var thrust_amt: float = 0
var rotation_direction: float = 0
var canvas_size: Vector2

@onready var collision_polygon: CollisionPolygon2D = $CollisionPolygon2D
@onready var ship_polygon: Polygon2D = $Polygon2D


func _ready():
	collision_polygon.polygon = ship_polygon.polygon


func _integrate_forces(state: PhysicsDirectBodyState2D) -> void:
	if thrust_amt != 0:
		state.apply_force(thrust_amt * thrust_magnitude * Vector2.UP.rotated(rotation))

	state.angular_velocity = rotation_direction * rotation_speed

	if state.linear_velocity.length() > max_speed:
		state.linear_velocity = state.linear_velocity.normalized() * max_speed

	if position.x > canvas_size.x:
		position.x = position.x - canvas_size.x
	elif position.x < 0:
		position.x = canvas_size.x - position.x

	if position.y > canvas_size.y:
		position.y = position.y - canvas_size.y
	elif position.y < 0:
		position.y = canvas_size.y - position.y


func _unhandled_input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_up"):
		thrust_amt = 1.0
	if event.is_action_released("ui_up"):
		thrust_amt = 0.0

	if event.is_action_pressed("ui_right"):
		rotation_direction += 1
	if event.is_action_released("ui_right"):
		rotation_direction -= 1

	if event.is_action_pressed("ui_left"):
		rotation_direction -= 1
	if event.is_action_released("ui_left"):
		rotation_direction += 1
