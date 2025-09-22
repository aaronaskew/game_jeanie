class_name EnemyCar
extends Car

var relative_velocity_y: float

@onready var player_car: PlayerCar = %PlayerCar


func _ready():
	super()
	accelerator_brake = -1.0


func _integrate_forces(state: PhysicsDirectBodyState2D) -> void:
	super(state)

	relative_velocity_y = state.linear_velocity.y - player_car.linear_velocity.y


func _physics_process(dt: float) -> void:
	position.y += relative_velocity_y * dt


func _on_area_2d_body_entered(body: Node2D) -> void:
	if body is PlayerCar:
		print("Player detected in direction ", position.direction_to(body.position))
