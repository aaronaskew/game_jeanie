extends Node2D

@onready var player_score_label: Label = %PlayerScore
@onready var ai_score_label: Label = %AIScore

var player_score: int = 0
var ai_score: int = 0

func _process(_delta: float) -> void:
    player_score_label.text = str(player_score)
    ai_score_label.text = str(ai_score)
