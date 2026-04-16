local PI = math.pi
local TAU = PI*2

local BALL_SIZE = luamm.Vec2.new(50, 50)
local BALL_SPEED = 20
local PADDLE_SIZE = luamm.Vec2.new(50, 200)
local PADDLE_SPEED = 20

local team = {
	NONE = 0,
	RED = 1,
	BLUE = 2,
}

local game_state = {
	paddle1 = luamm.Rect.new(luamm.Vec2.new(10, 50), PADDLE_SIZE),
	paddle2 = luamm.Rect.new(luamm.Vec2.new(400, 50), PADDLE_SIZE),
	ball = luamm.Rect.new(luamm.Vec2.new(100, 100), BALL_SIZE),
	direction = TAU*(7/8),
	bounces = { false, false, false, false },

	loser = team.NONE
}


function start()
	print("start")
end

function game_state:bounce(edge)
	if edge == luamm.Edge.LEFT or edge == luamm.Edge.RIGHT then
		self.direction = (PI-self.direction)%TAU
	elseif edge == luamm.Edge.TOP or edge == luamm.Edge.BOTTOM then
		self.direction = (TAU-self.direction)%TAU
	end
end

function game_state:set_bounce(side)
	self.bounces[1] = false
	self.bounces[2] = false
	self.bounces[3] = false
	self.bounces[4] = false
	self.bounces[side] = true
end

function game_state:reset()
	self.loser = team.NONE
	self.ball = luamm.Rect.new(luamm.Vec2.new(100, 100), BALL_SIZE)
	self.direction = TAU*(7/8)
	self.bounces = { false, false, false, false }
end

function process(delta)
	local screen = luamm.get_screen()
	game_state.paddle2.point.x = screen.x-60

	if luamm.is_key_down(luamm.Key.ENTER) then
		game_state:reset()
	end

	if game_state.loser == team.RED then
		local half_screen = luamm.Vec2.new(screen.x/2, screen.y/2)
		luamm.draw_text_center(half_screen, 120, "RED LOST", luamm.Color.WHITE)
		return
	elseif game_state.loser == team.BLUE then
		local half_screen = luamm.Vec2.new(screen.x/2, screen.y/2)
		luamm.draw_text_center(half_screen, 120, "BLUE LOST", luamm.Color.WHITE)
		return
	end

	local wasd = luamm.get_wasd()
	local arrows = luamm.get_arrows()

	game_state.paddle1.point.y = math.clamp(game_state.paddle1.point.y + wasd.y * delta * PADDLE_SPEED, 0, screen.y - PADDLE_SIZE.y)
	game_state.paddle2.point.y = math.clamp(game_state.paddle2.point.y + arrows.y * delta * PADDLE_SPEED, 0, screen.y - PADDLE_SIZE.y)

	luamm.draw(game_state.paddle1, luamm.Color.RED)
	luamm.draw(game_state.paddle2, luamm.Color.BLUE)

	local move_vec = luamm.Vec2.new(math.cos(game_state.direction), -math.sin(game_state.direction))

	game_state.ball.point += move_vec * delta * BALL_SPEED

	if game_state.ball:overlaps(game_state.paddle1) and not game_state.bounces[3] then
		game_state:set_bounce(3)
		game_state:bounce(luamm.Edge.LEFT)
	elseif game_state.ball:overlaps(game_state.paddle2) and not game_state.bounces[1] then
		game_state:set_bounce(1)
		game_state:bounce(luamm.Edge.RIGHT)
	elseif game_state.ball:get_edge(luamm.Edge.TOP) < 0 and not game_state.bounces[2] then
		game_state:set_bounce(2)
		game_state:bounce(luamm.Edge.TOP)
	elseif game_state.ball:get_edge(luamm.Edge.BOTTOM) > screen.y and not game_state.bounces[4] then
		game_state:set_bounce(4)
		game_state:bounce(luamm.Edge.BOTTOM)
	end

	if game_state.ball:get_edge(luamm.Edge.LEFT) < 0 then
		game_state.loser = team.RED
	elseif game_state.ball:get_edge(luamm.Edge.RIGHT) > screen.x then
		game_state.loser = team.BLUE
	end

	luamm.draw(game_state.ball, luamm.Color.WHITE)
end
