class_name CutScenes
extends Node2D

var panels: Array[Node]
var current_panel_idx


func _ready():
	panels = get_node("Panels").get_children()
	current_panel_idx = 0


func _process(_delta: float) -> void:
	for i in panels.size():
		var current_panel = panels[i] as Sprite2D
		if i == current_panel_idx:
			current_panel.visible = true
		else:
			current_panel.visible = false


func _unhandled_input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_left"):
		current_panel_idx = (current_panel_idx + panels.size() - 1) % panels.size()
		print(panels[current_panel_idx])
	if event.is_action_pressed("ui_right"):
		current_panel_idx = (current_panel_idx + panels.size() + 1) % panels.size()
		print(panels[current_panel_idx])