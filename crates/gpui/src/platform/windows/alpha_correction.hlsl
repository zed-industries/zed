float color_brightness(float3 color) {
    // REC. 601 luminance coefficients for perceived brightness
    return dot(color, float3(0.30f, 0.59f, 0.11f));
}

float light_on_dark_contrast(float enhancedContrast, float3 color) {
    float brightness = color_brightness(color);
    float multiplier = saturate(4.0f * (0.75f - brightness));
    return enhancedContrast * multiplier;
}

float enhance_contrast(float alpha, float k) {
    return alpha * (k + 1.0f) / (alpha * k + 1.0f);
}

float apply_alpha_correction(float a, float b, float4 g) {
    float brightness_adjustment = g.x * b + g.y;
    float correction = brightness_adjustment * a + (g.z * b + g.w);
    return a + a * (1.0f - a) * correction;
}

float apply_contrast_and_gamma_correction(float sample, float3 color, float enhanced_contrast_factor, float4 gamma_ratios) {
    float enhanced_contrast = light_on_dark_contrast(enhanced_contrast_factor, color);
    float brightness = color_brightness(color);

    float contrasted = enhance_contrast(sample, enhanced_contrast);
    return apply_alpha_correction(contrasted, brightness, gamma_ratios);
}
