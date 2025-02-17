extends Node


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var a = Main.new()
	
	print(a.sqrt(4))


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
