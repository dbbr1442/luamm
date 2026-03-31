local luamm = require("@luamm")

function start() 
	print("start")
end

function process()
	luamm.clear_screen(vector.create(255, 0 ,0))
end
