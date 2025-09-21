class_name Car
extends RigidBody2D

@export var force: Vector2 = Vector2(5000, 5000)
@export var max_speed: Vector2 = Vector2(2000.0, 500.0)
@export var braking_force: float = 2000.0

var accelerator_brake: float = 0
var direction: float = 0

var init_y_position: float

@onready var animated_sprite: AnimatedSprite2D = $AnimatedSprite2D
@onready var audio_collision: AudioStreamPlayer = $AudioStreamPlayer


func _ready():
	init_y_position = position.y


func _process(_delta):
	handle_input()

	if is_zero_approx(linear_velocity.x):
		animated_sprite.animation = "default"
	elif linear_velocity.x < 0:
		animated_sprite.animation = "left"
	else:
		animated_sprite.animation = "right"


func _integrate_forces(state: PhysicsDirectBodyState2D) -> void:
	state.apply_force(
		Vector2(
			force.x * direction,
			(
				force.y * accelerator_brake
				if accelerator_brake < 0
				else braking_force * accelerator_brake
			)
		)
	)

	if direction == 0:
		state.linear_velocity.x = 0

	state.linear_velocity = state.linear_velocity.clamp(-max_speed, max_speed)
	state.linear_velocity.y = clampf(state.linear_velocity.y, -max_speed.y, 0)

	position.y = init_y_position


func handle_input():
	direction = Input.get_axis("ui_left", "ui_right")

	accelerator_brake = Input.get_axis("ui_up", "ui_down")


func _on_body_entered(_body: Node) -> void:
	audio_collision.play()
