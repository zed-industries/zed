
compute:
	xcrun -sdk macosx metal -c examples/compute/shaders.metal -o examples/compute/shaders.air
	xcrun -sdk macosx metallib examples/compute/shaders.air -o examples/compute/shaders.metallib

window:
	xcrun -sdk macosx metal -c examples/window/shaders.metal -o examples/window/shaders.air
	xcrun -sdk macosx metallib examples/window/shaders.air -o examples/window/shaders.metallib

circle:
	xcrun -sdk macosx metal -c examples/circle/shaders.metal -o examples/circle/shaders.air
	xcrun -sdk macosx metallib examples/circle/shaders.air -o examples/circle/shaders.metallib

raytracing:
	xcrun -sdk macosx metal -c -g examples/raytracing/shaders.metal -o examples/raytracing/shaders.air
	xcrun -sdk macosx metallib examples/raytracing/shaders.air -o examples/raytracing/shaders.metallib
