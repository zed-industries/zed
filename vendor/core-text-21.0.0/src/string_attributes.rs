use core_foundation::string::CFStringRef;

extern "C" {
    pub static kCTCharacterShapeAttributeName: CFStringRef;
    pub static kCTFontAttributeName: CFStringRef;
    pub static kCTKernAttributeName: CFStringRef;
    pub static kCTLigatureAttributeName: CFStringRef;
    pub static kCTForegroundColorAttributeName: CFStringRef;
    pub static kCTForegroundColorFromContextAttributeName: CFStringRef;
    pub static kCTParagraphStyleAttributeName: CFStringRef;
    pub static kCTStrokeWidthAttributeName: CFStringRef;
    pub static kCTStrokeColorAttributeName: CFStringRef;
    pub static kCTSuperscriptAttributeName: CFStringRef;
    pub static kCTUnderlineColorAttributeName: CFStringRef;
    pub static kCTUnderlineStyleAttributeName: CFStringRef;
    pub static kCTVerticalFormsAttributeName: CFStringRef;
    pub static kCTGlyphInfoAttributeName: CFStringRef;
    pub static kCTRunDelegateAttributeName: CFStringRef;
}
