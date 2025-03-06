---@diagnostic disable: undefined-global

-- Create a sandbox environment
local sandbox = {}

-- Allow access to standard libraries (safe subset)
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

-- Create a sandboxed version of LuaFileIO
local io = {}

-- File functions
io.open = sb_io_open

-- Add the sandboxed io library to the sandbox environment
sandbox.io = io


-- Load the script with the sandbox environment
local user_script_fn, err = load(user_script, nil, "t", sandbox)

if not user_script_fn then
  error("Failed to load user script: " .. tostring(err))
end

-- Execute the user script within the sandbox
local success, result = pcall(user_script_fn)

if not success then
  error("Error executing user script: " .. tostring(result))
end
