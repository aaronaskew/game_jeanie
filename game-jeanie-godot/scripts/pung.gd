extends Node2D

@onready var player_score_label: Label = %PlayerScore
@onready var ai_score_label: Label = %AIScore
@onready var player_paddle = %PlayerPaddle
@onready var ai_paddle = %AIPaddle
@onready var ball = $Ball


var player_score: int = 0
var ai_score: int = 0

func _process(_delta: float) -> void:
    player_score_label.text = str(player_score)
    ai_score_label.text = str(ai_score)

    # Move player
    if Input.is_action_pressed("ui_up"):
        player_paddle.move_y(-1)

    if Input.is_action_pressed("ui_down"):
        player_paddle.move_y(1)


    # Move AI
    var direction: float = ball.position.y - ai_paddle.position.y
    if direction > 0:
        ai_paddle.move_y(1)
    elif direction < 0:
        ai_paddle.move_y(-1)