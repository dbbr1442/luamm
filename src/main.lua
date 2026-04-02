local luamm = require("@luamm")

function start() 
	print("start")
end

function process()
	luamm.clear_screen(luamm.Color.new(255, 0, 0))
	luamm.draw_rect(luamm.Vec2.new(20, 20), luamm.Vec2.new(500, 500), luamm.Color.new(0, 255, 0));
end
