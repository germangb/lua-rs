print "hello vector"

local v = vec.new()

vec.add(v, 0)
vec.add(v, 1)
vec.add(v, 2)
vec.add(v, 3)
vec.add(v, 4)

print("vector:", v)

print("vector[3] =", vec.get(v, 3))


-- Runtime error (out of bounds):
-- print("vector[42] =", vec.get(v, 42))
