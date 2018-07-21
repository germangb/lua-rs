--print("running lua script")

foo = {
    bar = 42,

    -- closure
    hello = function ()
        print("hello!!!!")
    end
}

function fun()
    bar = "global"
end
