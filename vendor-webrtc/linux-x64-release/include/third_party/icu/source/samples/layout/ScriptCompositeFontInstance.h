/*
 * %W% %E%
 *
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 *
 * (C) Copyright IBM Corp. 1998-2011 - All Rights Reserved
 *
 */

#ifndef __SCRIPTCOMPOSITEFONTINSTANCE_H
#define __SCRIPTCOMPOSITEFONTINSTANCE_H

#include "layout/LETypes.h"
#include "layout/LEFontInstance.h"

#include "FontMap.h"

// U_NAMESPACE_BEGIN

class ScriptCompositeFontInstance : public LEFontInstance
{
public:

    ScriptCompositeFontInstance(FontMap *fontMap);

    virtual ~ScriptCompositeFontInstance();

      /**
     * Get a physical font which can render the given text. For composite fonts,
     * if there is no single physical font which can render all of the text,
     * return a physical font which can render an initial substring of the text,
     * and set the <code>offset</code> parameter to the end of that substring.
     *
     * Internally, the LayoutEngine works with runs of text all in the same
     * font and script, so it is best to call this method with text which is
     * in a single script, passing the script code in as a hint. If you don't
     * know the script of the text, you can use zero, which is the script code
     * for characters used in more than one script.
     *
     * The default implementation of this method is intended for instances of
     * <code>LEFontInstance</code> which represent a physical font. It returns
     * <code>this</code> and indicates that the entire string can be rendered.
     *
     * This method will return a valid <code>LEFontInstance</code> unless you
     * have passed illegal parameters, or an internal error has been encountered. 
     * For composite fonts, it may return the warning <code>LE_NO_SUBFONT_WARNING</code>
     * to indicate that the returned font may not be able to render all of
     * the text. Whenever a valid font is returned, the <code>offset</code> parameter
     * will be advanced by at least one.
     *
     * Subclasses which implement composite fonts must override this method.
     * Where it makes sense, they should use the script code as a hint to render
     * characters from the COMMON script in the font which is used for the given
     * script. For example, if the input text is a series of Arabic words separated
     * by spaces, and the script code passed in is <code>arabScriptCode</code> you
     * should return the font used for Arabic characters for all of the input text,
     * including the spaces. If, on the other hand, the input text contains characters
     * which cannot be rendered by the font used for Arabic characters, but which can
     * be rendered by another font, you should return that font for those characters.
     *
     * @param chars   - the array of Unicode characters.
     * @param offset  - a pointer to the starting offset in the text. On exit this
     *                  will be set the the limit offset of the text which can be
     *                  rendered using the returned font.
     * @param limit   - the limit offset for the input text.
     * @param script  - the script hint.
     * @param success - set to an error code if the arguments are illegal, or no font
     *                  can be returned for some reason. May also be set to
     *                  <code>LE_NO_SUBFONT_WARNING</code> if the subfont which
     *                  was returned cannot render all of the text.
     *
     * @return an <code>LEFontInstance</code> for the sub font which can render the characters, or
     *         <code>nullptr</code> if there is an error.
     *
     * @see LEScripts.h
     */
    virtual const LEFontInstance *getSubFont(const LEUnicode chars[], le_int32 *offset, le_int32 limit, le_int32 script, LEErrorCode &success) const;

    /**
     * This method maps a single character to a glyph index, using the
     * font's character to glyph map.
     *
     * @param ch - the character
     *
     * @return the glyph index
     */
    virtual LEGlyphID mapCharToGlyph(LEUnicode32 ch) const;

    virtual const void *getFontTable(LETag tableTag) const;

    virtual le_int32 getUnitsPerEM() const;

    virtual le_int32 getAscent() const;

    virtual le_int32 getDescent() const;

    virtual le_int32 getLeading() const;

    virtual void getGlyphAdvance(LEGlyphID glyph, LEPoint &advance) const;

    virtual le_bool getGlyphPoint(LEGlyphID glyph, le_int32 pointNumber, LEPoint &point) const;

    float getXPixelsPerEm() const;

    float getYPixelsPerEm() const;

    float getScaleFactorX() const;

    float getScaleFactorY() const;

    /**
     * ICU "poor man's RTTI", returns a UClassID for the actual class.
     */
    virtual inline UClassID getDynamicClassID() const { return getStaticClassID(); }

    /**
     * ICU "poor man's RTTI", returns a UClassID for this class.
     */
    static inline UClassID getStaticClassID() { return (UClassID)&fgClassID; }

protected:
    FontMap *fFontMap;

private:

    /**
     * The address of this static class variable serves as this class's ID
     * for ICU "poor man's RTTI".
     */
    static const char fgClassID;
};

inline const void *ScriptCompositeFontInstance::getFontTable(LETag /*tableTag*/) const
{
    return nullptr;
}

// Can't get units per EM without knowing which sub-font, so
// return a value that will make units == points
inline le_int32 ScriptCompositeFontInstance::getUnitsPerEM() const
{
    return 1;
}

inline le_int32 ScriptCompositeFontInstance::getAscent() const
{
    return fFontMap->getAscent();
}

inline le_int32 ScriptCompositeFontInstance::getDescent() const
{
    return fFontMap->getDescent();
}

inline le_int32 ScriptCompositeFontInstance::getLeading() const
{
    return fFontMap->getLeading();
}

inline float ScriptCompositeFontInstance::getXPixelsPerEm() const
{
    return fFontMap->getPointSize();
}

inline float ScriptCompositeFontInstance::getYPixelsPerEm() const
{
    return fFontMap->getPointSize();
}

// Can't get a scale factor without knowing the sub-font, so
// return 1.0.
inline float ScriptCompositeFontInstance::getScaleFactorX() const
{
    return 1.0;
}

// Can't get a scale factor without knowing the sub-font, so
// return 1.0
inline float ScriptCompositeFontInstance::getScaleFactorY() const
{
    return 1.0;
}

// U_NAMESPACE_END
#endif
