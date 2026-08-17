#!/usr/bin/env python3

# compute arctangent table for CORDIC computations in fttrigon.c
import math

# units  = 64*65536.0   # don't change !!
units = 180 * 2 ** 16
scale = units / math.pi
shrink = 1.0
angles2 = []

print("")
print("table of arctan( 1/2^n ) for PI = " + repr(units / 65536.0) + " units")

for n in range(1, 32):

    x = 0.5 ** n  # tangent value

    angle = math.atan(x)  # arctangent
    angle2 = round(angle * scale)  # arctangent in FT_Angle units

    if angle2 <= 0:
        break

    angles2.append(repr(int(angle2)))
    shrink /= math.sqrt(1 + x * x)

print(", ".join(angles2))
print("shrink factor    = " + repr(shrink))
print("shrink factor 2  = " + repr(int(shrink * (2 ** 32))))
print("expansion factor = " + repr(1 / shrink))
print("")
