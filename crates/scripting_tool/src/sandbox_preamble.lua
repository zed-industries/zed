---@diagnostic disable: undefined-global

local sandbox = {}

sandbox.string = string
sandbox.table = table
sandbox.math = math
sandbox.print = sb_print
sandbox.type = type
sandbox.tostring = tostring
sandbox.tonumber = tonumber
sandbox.pairs = pairs
sandbox.ipairs = ipairs
sandbox.search = search

local io = {}

io.open = sb_io_open
io.popen = sb_io_popen

sandbox.io = io

local user_script_fn, err = load(user_script, nil, "t", sandbox)

if not user_script_fn then
  error("Failed to load user script: " .. tostring(err))
end

local success, result = pcall(user_script_fn)

if not success then
  error("Error executing user script: " .. tostring(result))
end
