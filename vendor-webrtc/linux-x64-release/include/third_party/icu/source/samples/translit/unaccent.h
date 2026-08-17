/***********************************************************************
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 ***********************************************************************
 ***********************************************************************
 * COPYRIGHT:
 * Copyright (c) 1999-2003, International Business Machines Corporation and
 * others. All Rights Reserved.
 ***********************************************************************/

#include "unicode/translit.h"
#include "unicode/normlzr.h"

using namespace icu;

class UnaccentTransliterator : public Transliterator {
    
 public:
    
    /**
     * Constructor
     */
    UnaccentTransliterator();

    /**
     * Destructor
     */
    virtual ~UnaccentTransliterator();

 protected:

    /**
     * Implement Transliterator API
     */
    virtual void handleTransliterate(Replaceable& text,
                                     UTransPosition& index,
                                     UBool incremental) const;

 private:

    /**
     * Unaccent a single character using normalizer.
     */
    char16_t unaccent(char16_t c) const;

    Normalizer normalizer;

public:

    /**
     * Return the class ID for this class.  This is useful only for
     * comparing to a return value from getDynamicClassID().  For example:
     * <pre>
     * .      Base* polymorphic_pointer = createPolymorphicObject();
     * .      if (polymorphic_pointer->getDynamicClassID() ==
     * .          Derived::getStaticClassID()) ...
     * </pre>
     * @return          The class ID for all objects of this class.
     * @stable ICU 2.0
     */
    static inline UClassID getStaticClassID() { return (UClassID)&fgClassID; };

    /**
     * Returns a unique class ID <b>polymorphically</b>.  This method
     * is to implement a simple version of RTTI, since not all C++
     * compilers support genuine RTTI.  Polymorphic operator==() and
     * clone() methods call this method.
     * 
     * <p>Concrete subclasses of Transliterator that wish clients to
     * be able to identify them should implement getDynamicClassID()
     * and also a static method and data member:
     * 
     * <pre>
     * static UClassID getStaticClassID() { return (UClassID)&fgClassID; }
     * static char fgClassID;
     * </pre>
     *
     * Subclasses that do not implement this method will have a
     * dynamic class ID of Transliterator::getStatisClassID().
     *
     * @return The class ID for this object. All objects of a given
     * class have the same class ID.  Objects of other classes have
     * different class IDs.
     * @stable ICU 2.0
     */
    virtual UClassID getDynamicClassID() const { return getStaticClassID(); };

private:

    /**
     * Class identifier for subclasses of Transliterator that do not
     * define their class (anonymous subclasses).
     */
    static const char fgClassID;
};
