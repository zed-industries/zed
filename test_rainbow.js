// Test file for rainbow brackets
function test() {
    const arr = [1, 2, 3];
    const obj = {
        key: "value",
        nested: {
            deep: [4, 5, 6]
        }
    };
    
    if (true) {
        console.log("Hello");
        for (let i = 0; i < 10; i++) {
            console.log(i);
        }
    }
    
    return arr.map((x) => x * 2);
}

// More nested structures
const complex = {
    a: [
        {
            b: [
                {
                    c: [1, 2, 3]
                }
            ]
        }
    ]
};