class_name Ship
extends Area2D

# pub SHIP_THRUST_MAGNITUDE: f32,
# pub SHIP_MAX_VELOCITY: f32,
# pub SHIP_ROTATION_SPEED: f32,
# pub SHIP_INVINCIBLE_TIME: f32,
# pub SHIP_BLINK_RATE: f32,

signal make_death_process

@export var thrust_magnitude: float = 500.0
@export var max_speed: float = 1000.0
@export var rotation_speed: float = 0.5 * TAU
@export var invincible_time: float = 3.0
@export var blink_rate: float = 50

var thrust_amt: float = 0
var rotation_direction: float = 0
var canvas_size: Vector2
var is_invincible = false
var time_passed: float = 0
var velocity: Vector2
var blaster_bullet_scene = preload("res://scenes/beef_blastoids/blaster_bullet.tscn")
var ship_explosion_scene = preload("res://scenes/beef_blastoids/ship_explosion.tscn")

@onready var collision_polygon: CollisionPolygon2D = $CollisionPolygon2D
@onready var ship_polygon: Polygon2D = $Polygon2D
@onready var invincible_timer: Timer = $Timer


func _ready():
	collision_polygon.polygon = ship_polygon.polygon

	is_invincible = true
	collision_polygon.disabled = true
	invincible_timer.wait_time = invincible_time
	invincible_timer.start()


func _physics_process(dt: float) -> void:
	rotation += rotation_direction * rotation_speed * dt

	velocity += thrust_amt * thrust_magnitude * Vector2.UP.rotated(rotation) * dt

	if velocity.length() > max_speed:
		velocity = velocity.normalized() * max_speed

	position += velocity * dt

	if position.x > canvas_size.x:
		position.x = position.x - canvas_size.x
	elif position.x < 0:
		position.x = canvas_size.x - position.x

	if position.y > canvas_size.y:
		position.y = position.y - canvas_size.y
	elif position.y < 0:
		position.y = canvas_size.y - position.y

	if has_overlapping_bodies():
		if !is_invincible:
			var ship_explosion: GPUParticles2D = ship_explosion_scene.instantiate()
			ship_explosion.position = position
			add_sibling(ship_explosion, true)
			ship_explosion.emitting = true
			make_death_process.emit(ship_explosion)
			queue_free()


func _process(dt: float) -> void:
	time_passed += dt
	if is_invincible:
		blink()

	handle_input()


func fire_blaster():
	var blaster_bullet: BlasterBullet = blaster_bullet_scene.instantiate()
	blaster_bullet.initialize(position, rotation)
	add_sibling(blaster_bullet, true)


func blink():
	if sin(time_passed * blink_rate * 2 * PI) > 0:
		ship_polygon.visible = false
	else:
		ship_polygon.visible = true


func handle_input() -> void:
	rotation_direction = Input.get_axis("ui_left", "ui_right")
	thrust_amt = Input.get_action_strength("ui_up")

	if Input.is_action_just_pressed("ui_accept"):
		fire_blaster()


func _on_timer_timeout() -> void:
	collision_polygon.disabled = false
	ship_polygon.visible = true
	is_invincible = false
