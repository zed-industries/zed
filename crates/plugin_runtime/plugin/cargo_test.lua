query = [[(
    (attribute_item
        (meta_item
            (identifier) @test)) @attribute
    .
    (function_item
        name: (identifier) @name) @funciton
)]]

function run_test(name)
    local command = 'cargo test -- ' .. name
    local openPop = assert(io.popen(command, 'r'))
    local output = openPop:read('*all')
    openPop:close()
    return output
end
