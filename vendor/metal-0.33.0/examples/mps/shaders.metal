//
//  Created by Sergey Reznik on 9/15/18.
//  Copyright © 2018 Serhii Rieznik. All rights reserved.
//

// Taken from https://github.com/sergeyreznik/metal-ray-tracer/tree/part-1/source/Shaders
// MIT License https://github.com/sergeyreznik/metal-ray-tracer/blob/part-1/LICENSE

#include <MetalPerformanceShaders/MetalPerformanceShaders.h>

using Ray = MPSRayOriginMinDistanceDirectionMaxDistance;
using Intersection = MPSIntersectionDistancePrimitiveIndexCoordinates;

kernel void generateRays(
    device Ray* rays [[buffer(0)]],
    uint2 coordinates [[thread_position_in_grid]],
    uint2 size [[threads_per_grid]])
{
    float2 uv = float2(coordinates) / float2(size - 1);

    uint rayIndex = coordinates.x + coordinates.y * size.x;
    rays[rayIndex].origin = MPSPackedFloat3(uv.x, uv.y, -1.0);
    rays[rayIndex].direction = MPSPackedFloat3(0.0, 0.0, 1.0);
    rays[rayIndex].minDistance = 0.0f;
    rays[rayIndex].maxDistance = 2.0f;
}
