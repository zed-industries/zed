---@diagnostic disable: undefined-global

-- Create a sandbox environment
local sandbox = {}

-- For now, add all globals to `sandbox` (so there effectively is no sandbox).
-- We still need the logic below so that we can do things like overriding print() to write
-- to our in-memory log rather than to stdout, we will delete this loop (and re-enable
-- the I/O module being sandboxed below) to have things be sandboxed again.
for k, v in pairs(_G) do
    if sandbox[k] == nil then
        sandbox[k] = v
    end
end

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

-- Access to custom functions
sandbox.search = search
sandbox.outline = outline

-- Create a sandboxed version of LuaFileIO
-- local io = {};
--
-- For now we are using unsandboxed io
local io = _G.io;

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
