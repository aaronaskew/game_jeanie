class_name PlayerCar
extends Car

signal turn

@export var fuel_effeciency: float = 100.0
@export var max_fuel: float = 5000.0

var current_fuel: float
# var total_distance_traveled: float = 0
var current_animation: String = "default"
var init_y_position: float
var virtual_y_pos: float = 0.0

@onready var audio_collision: AudioStreamPlayer = $AudioStreamPlayer


func _ready():
	super()
	init_y_position = position.y
	current_fuel = max_fuel


func _process(dt):
	super(dt)

	handle_input()

	if current_animation != animated_sprite.animation:
		current_animation = animated_sprite.animation
		if current_animation != "default":
			turn.emit()


func _physics_process(dt):
	var distance_traveled: float = abs(linear_velocity.y) * dt
	# total_distance_traveled += distance_traveled
	current_fuel = clampf(current_fuel - distance_traveled / fuel_effeciency, 0, max_fuel)

	virtual_y_pos += linear_velocity.y * dt


func _integrate_forces(state: PhysicsDirectBodyState2D) -> void:
	if current_fuel <= 0 && accelerator_brake < 0:
		accelerator_brake = 0

	super(state)

	# prevent player from going in reverse
	state.linear_velocity.y = clampf(state.linear_velocity.y, -max_speed.y, 0)
	# lock player's y position
	position.y = init_y_position


func _on_body_entered(_body: Node) -> void:
	audio_collision.play()


func handle_input():
	direction = Input.get_axis("ui_left", "ui_right")
	accelerator_brake = Input.get_axis("ui_up", "ui_down")
