---@meta luamm

luamm = {}

-- Vec2 definitions

---@class Vec2
---@field x number
---@field y number
luamm.Vec2INST = {}

luamm.Vec2 = {
	---@type Vec2
	ZERO = {},
	---@type Vec2
	UP = {},
	---@type Vec2
	DOWN = {},
	---@type Vec2
	LEFT = {},
	---@type Vec2
	RIGHT = {}
}

---@param x number
---@param y number
---@return Vec2
function luamm.Vec2.new(x, y) end




-- Color definitions

---@class Color
---@field r number
---@field g number
---@field b number
luamm.ColorINST = {}

luamm.Color = {}

---@type Color
luamm.Color.WHITE = {}

---@type Color
luamm.Color.BLACK = {}

---@type Color
luamm.Color.RED = {}

---@type Color
luamm.Color.GREEN = {}

---@type Color
luamm.Color.BLUE = {}

---@param r number
---@param g number
---@param b number
---@return Color
function luamm.Color.new_rgb(r, g, b) end



-- Key definitions

---@class Key
luamm.KeyINST = {}

luamm.Key = {
	---@type Key
	W		  =	   {},
	---@type Key
	A		  =	   {},
	---@type Key
	S		  =	   {},
	---@type Key
	D		  =	   {},
	---@type Key
	UP		  =	   {},
	---@type Key
	LEFT	  =	   {},
	---@type Key
	RIGHT	  =	   {},
	---@type Key
	DOWN	  =	   {},
	---@type Key
	Q		  =	   {},
	---@type Key
	E		  =	   {},
	---@type Key
	ESCAPE	  =	   {},
	---@type Key
	ENTER	  =	   {},
	---@type Key
}


---@param pos Vec2
---@param size Vec2
---@param color Color
function luamm.draw_rect(pos, size, color) end

---@param key Key
function luamm.is_key_down(key) end

---@return Vec2
function luamm.get_wasd() end

---@return Vec2
function luamm.get_screen() end

function luamm.close() end

---@return Key[]
function luamm.get_keys_down() end

---@param n number
---@param min number
---@param max number
---@return number
function math.clamp(n, min, max) end

function luamm.sleep() end
---@param color Color
function luamm.clear_screen(color) end

---@class Edge
luamm.EdgeINST = {}

luamm.Edge = {
	---@type Edge
	TOP = {},
	---@type Edge
	BOTTOM = {},
	---@type Edge
	LEFT = {},
	---@type Edge
	RIGHT = {},
}

---@class Rect 
---@field point Vec2
---@field size Vec2
luamm.RectINST = {}

---@return number
---@param edge Edge
function luamm.RectINST:get_edge(edge) end

---@return boolean
---@param other Rect
function luamm.RectINST:overlaps(other) end

luamm.Rect = {}

---@return Rect
---@param point Vec2
---@param size Vec2
function luamm.Rect.new(point, size) end

---@param color Color
function luamm.draw(shape, color) end

---@return Vec2
function luamm.get_arrows() end

---@param location Vec2
---@param size integer
---@param text string
---@param color Color
function luamm.draw_text(location, size, text, color) end

---@param location Vec2
---@param size integer
---@param text string
---@param color Color
function luamm.draw_text_center(location, size, text, color) end
