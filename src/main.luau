local luamm = require("@luamm")

function start() 
	print("start")
end

loc = luamm.Vec2.new(20, 20)

function process()
	luamm.clear_screen(luamm.Color.new(255, 0, 0))
	
	local wasd = luamm.get_wasd()

	loc += wasd * 10

	local screen_size = luamm.get_screen()

	loc.x = math.clamp(loc.x, 0, screen_size.x-500)
	loc.y = math.clamp(loc.y, 0, screen_size.y-500)

	if luamm.is_key_down(luamm.Key.new("esc")) then
		luamm.close()
	end

	local color2 = luamm.Color.new(0, 0, 255)
	luamm.draw_rect(loc, luamm.Vec2.new(500, 500), color2);
end
