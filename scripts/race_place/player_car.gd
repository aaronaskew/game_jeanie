class_name PlayerCar
extends Car

signal turn

@export var fuel_effeciency: float = 1.0
@export var max_fuel: float = 100.0

var distance_traveled: float = 0
var current_animation: String = "default"

@onready var audio_collision: AudioStreamPlayer = $AudioStreamPlayer


func _process(dt):
	super(dt)

	handle_input()

	if current_animation != animated_sprite.animation:
		current_animation = animated_sprite.animation
		if current_animation != "default":
			turn.emit()

	distance_traveled += abs(linear_velocity.y) * dt


func _on_body_entered(_body: Node) -> void:
	audio_collision.play()


func handle_input():
	direction = Input.get_axis("ui_left", "ui_right")
	accelerator_brake = Input.get_axis("ui_up", "ui_down")
