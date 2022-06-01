print("initializing plugin...")

query = [[(
    (attribute_item
        (meta_item
            (identifier) @test)) @attribute
    .
    (function_item
        name: (identifier) @name) @funciton
)]]

function run_test(name)
    print('running test `' .. name .. '`:')
    local command = 'cargo test -- ' .. name
    local openPop = assert(io.popen(command, 'r'))
    local output = openPop:read('*all')
    openPop:close()
    print('done running test')
    return output
end

print("done initializing plugin.")