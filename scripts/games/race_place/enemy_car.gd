class_name EnemyCar
extends Car

@onready var player_car: PlayerCar = %PlayerCar
@onready var init_y_position: float = position.y


func _ready():
	super()
	accelerator_brake = -1.0


func _integrate_forces(state: PhysicsDirectBodyState2D) -> void:
	super(state)


func _physics_process(dt: float) -> void:
	super(dt)

	position.y = init_y_position + virtual_y_pos - player_car.virtual_y_pos


func _on_area_2d_body_entered(body: Node2D) -> void:
	if body is PlayerCar:
		print("Player detected in direction ", position.direction_to(body.position))
