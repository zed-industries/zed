// clang-format off
#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_GRAMMAR_GENERATED_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_GRAMMAR_GENERATED_H_
// A Bison parser, made by GNU Bison 3.8.2.

// Skeleton interface for Bison LALR(1) parsers in C++

// Copyright (C) 2002-2015, 2018-2021 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// As a special exception, you may create a larger work that contains
// part or all of the Bison parser skeleton and distribute that work
// under terms of your choice, so long as that work isn't itself a
// parser generator using the skeleton or a modified version thereof
// as a parser skeleton.  Alternatively, if you modify or redistribute
// the parser skeleton itself, you may (at your option) remove this
// special exception, which will cause the skeleton and the resulting
// Bison output files to be licensed under the GNU General Public
// License without this special exception.

// This special exception was added by the Free Software Foundation in
// version 2.2 of Bison.


/**
 ** \file third_party/blink/renderer/core/xml/xpath_grammar_generated.h
 ** Define the xpathyy::parser class.
 */

// C++ LALR(1) parser skeleton written by Akim Demaille.

// DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
// especially those whose name start with YY_ or yy_.  They are
// private implementation details that can be changed or removed.

#ifndef YY_YY_THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_GRAMMAR_GENERATED_HH_INCLUDED
# define YY_YY_THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_GRAMMAR_GENERATED_HH_INCLUDED
// "%code requires" blocks.
#line 46 "third_party/blink/renderer/core/xml/xpath_grammar.y"


#include "third_party/blink/renderer/platform/heap/persistent.h"


#line 55 "third_party/blink/renderer/core/xml/xpath_grammar_generated.h"


# include <cstdlib> // std::abort
# include <iostream>
# include <stdexcept>
# include <string>
# include <vector>

#if defined __cplusplus
# define YY_CPLUSPLUS __cplusplus
#else
# define YY_CPLUSPLUS 199711L
#endif

// Support move semantics when possible.
#if 201103L <= YY_CPLUSPLUS
# define YY_MOVE           std::move
# define YY_MOVE_OR_COPY   move
# define YY_MOVE_REF(Type) Type&&
# define YY_RVREF(Type)    Type&&
# define YY_COPY(Type)     Type
#else
# define YY_MOVE
# define YY_MOVE_OR_COPY   copy
# define YY_MOVE_REF(Type) Type&
# define YY_RVREF(Type)    const Type&
# define YY_COPY(Type)     const Type&
#endif

// Support noexcept when possible.
#if 201103L <= YY_CPLUSPLUS
# define YY_NOEXCEPT noexcept
# define YY_NOTHROW
#else
# define YY_NOEXCEPT
# define YY_NOTHROW throw ()
#endif

// Support constexpr when possible.
#if 201703 <= YY_CPLUSPLUS
# define YY_CONSTEXPR constexpr
#else
# define YY_CONSTEXPR
#endif



#ifndef YY_ATTRIBUTE_PURE
# if defined __GNUC__ && 2 < __GNUC__ + (96 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_PURE __attribute__ ((__pure__))
# else
#  define YY_ATTRIBUTE_PURE
# endif
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# if defined __GNUC__ && 2 < __GNUC__ + (7 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_UNUSED __attribute__ ((__unused__))
# else
#  define YY_ATTRIBUTE_UNUSED
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YY_USE(E) ((void) (E))
#else
# define YY_USE(E) /* empty */
#endif

/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
#if defined __GNUC__ && ! defined __ICC && 406 <= __GNUC__ * 100 + __GNUC_MINOR__
# if __GNUC__ * 100 + __GNUC_MINOR__ < 407
#  define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                           \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")
# else
#  define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                           \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")              \
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# endif
# define YY_IGNORE_MAYBE_UNINITIALIZED_END      \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif

#if defined __cplusplus && defined __GNUC__ && ! defined __ICC && 6 <= __GNUC__
# define YY_IGNORE_USELESS_CAST_BEGIN                          \
    _Pragma ("GCC diagnostic push")                            \
    _Pragma ("GCC diagnostic ignored \"-Wuseless-cast\"")
# define YY_IGNORE_USELESS_CAST_END            \
    _Pragma ("GCC diagnostic pop")
#endif
#ifndef YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_END
#endif

# ifndef YY_CAST
#  ifdef __cplusplus
#   define YY_CAST(Type, Val) static_cast<Type> (Val)
#   define YY_REINTERPRET_CAST(Type, Val) reinterpret_cast<Type> (Val)
#  else
#   define YY_CAST(Type, Val) ((Type) (Val))
#   define YY_REINTERPRET_CAST(Type, Val) ((Type) (Val))
#  endif
# endif
# ifndef YY_NULLPTR
#  if defined __cplusplus
#   if 201103L <= __cplusplus
#    define YY_NULLPTR nullptr
#   else
#    define YY_NULLPTR 0
#   endif
#  else
#   define YY_NULLPTR ((void*)0)
#  endif
# endif

/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif

#line 76 "third_party/blink/renderer/core/xml/xpath_grammar.y"
namespace xpathyy {
#line 191 "third_party/blink/renderer/core/xml/xpath_grammar_generated.h"




  /// A Bison parser.
  class YyParser
  {
  public:
#ifdef YYSTYPE
# ifdef __GNUC__
#  pragma GCC message "bison: do not #define YYSTYPE in C++, use %define api.value.type"
# endif
    typedef YYSTYPE value_type;
#else
  /// A buffer to store and retrieve objects.
  ///
  /// Sort of a variant, but does not keep track of the nature
  /// of the stored data, since that knowledge is available
  /// via the current parser state.
  class value_type
  {
  public:
    /// Type of *this.
    typedef value_type self_type;

    /// Empty construction.
    value_type () YY_NOEXCEPT
      : yyraw_ ()
    {}

    /// Construct and fill.
    template <typename T>
    value_type (YY_RVREF (T) t)
    {
      new (yyas_<T> ()) T (YY_MOVE (t));
    }

#if 201103L <= YY_CPLUSPLUS
    /// Non copyable.
    value_type (const self_type&) = delete;
    /// Non copyable.
    self_type& operator= (const self_type&) = delete;
#endif

    /// Destruction, allowed only if empty.
    ~value_type () YY_NOEXCEPT
    {}

# if 201103L <= YY_CPLUSPLUS
    /// Instantiate a \a T in here from \a t.
    template <typename T, typename... U>
    T&
    emplace (U&&... u)
    {
      return *new (yyas_<T> ()) T (std::forward <U>(u)...);
    }
# else
    /// Instantiate an empty \a T in here.
    template <typename T>
    T&
    emplace ()
    {
      return *new (yyas_<T> ()) T ();
    }

    /// Instantiate a \a T in here from \a t.
    template <typename T>
    T&
    emplace (const T& t)
    {
      return *new (yyas_<T> ()) T (t);
    }
# endif

    /// Instantiate an empty \a T in here.
    /// Obsolete, use emplace.
    template <typename T>
    T&
    build ()
    {
      return emplace<T> ();
    }

    /// Instantiate a \a T in here from \a t.
    /// Obsolete, use emplace.
    template <typename T>
    T&
    build (const T& t)
    {
      return emplace<T> (t);
    }

    /// Accessor to a built \a T.
    template <typename T>
    T&
    as () YY_NOEXCEPT
    {
      return *yyas_<T> ();
    }

    /// Const accessor to a built \a T (for %printer).
    template <typename T>
    const T&
    as () const YY_NOEXCEPT
    {
      return *yyas_<T> ();
    }

    /// Swap the content with \a that, of same type.
    ///
    /// Both variants must be built beforehand, because swapping the actual
    /// data requires reading it (with as()), and this is not possible on
    /// unconstructed variants: it would require some dynamic testing, which
    /// should not be the variant's responsibility.
    /// Swapping between built and (possibly) non-built is done with
    /// self_type::move ().
    template <typename T>
    void
    swap (self_type& that) YY_NOEXCEPT
    {
      std::swap (as<T> (), that.as<T> ());
    }

    /// Move the content of \a that to this.
    ///
    /// Destroys \a that.
    template <typename T>
    void
    move (self_type& that)
    {
# if 201103L <= YY_CPLUSPLUS
      emplace<T> (std::move (that.as<T> ()));
# else
      emplace<T> ();
      swap<T> (that);
# endif
      that.destroy<T> ();
    }

# if 201103L <= YY_CPLUSPLUS
    /// Move the content of \a that to this.
    template <typename T>
    void
    move (self_type&& that)
    {
      emplace<T> (std::move (that.as<T> ()));
      that.destroy<T> ();
    }
#endif

    /// Copy the content of \a that to this.
    template <typename T>
    void
    copy (const self_type& that)
    {
      emplace<T> (that.as<T> ());
    }

    /// Destroy the stored \a T.
    template <typename T>
    void
    destroy ()
    {
      as<T> ().~T ();
    }

  private:
#if YY_CPLUSPLUS < 201103L
    /// Non copyable.
    value_type (const self_type&);
    /// Non copyable.
    self_type& operator= (const self_type&);
#endif

    /// Accessor to raw memory as \a T.
    template <typename T>
    T*
    yyas_ () YY_NOEXCEPT
    {
      void *yyp = yyraw_;
      return static_cast<T*> (yyp);
     }

    /// Const accessor to raw memory as \a T.
    template <typename T>
    const T*
    yyas_ () const YY_NOEXCEPT
    {
      const void *yyp = yyraw_;
      return static_cast<const T*> (yyp);
     }

    /// An auxiliary type to compute the largest semantic type.
    union union_type
    {
      // kNodeType
      // kPI
      // kFunctionName
      // kLiteral
      // kVariableReference
      // kNumber
      // kNameTest
      char dummy1[sizeof (String)];

      // ArgumentList
      char dummy2[sizeof (blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Expression>>>)];

      // OptionalPredicateList
      // PredicateList
      char dummy3[sizeof (blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Predicate>>>)];

      // Expr
      // Predicate
      // PrimaryExpr
      // FunctionCall
      // Argument
      // UnionExpr
      // PathExpr
      // FilterExpr
      // OrExpr
      // AndExpr
      // EqualityExpr
      // RelationalExpr
      // AdditiveExpr
      // MultiplicativeExpr
      // UnaryExpr
      char dummy4[sizeof (blink::Persistent<blink::xpath::Expression>)];

      // LocationPath
      // AbsoluteLocationPath
      // RelativeLocationPath
      char dummy5[sizeof (blink::Persistent<blink::xpath::LocationPath>)];

      // NodeTest
      char dummy6[sizeof (blink::Persistent<blink::xpath::Step::NodeTest>)];

      // Step
      // DescendantOrSelf
      // AbbreviatedStep
      char dummy7[sizeof (blink::Persistent<blink::xpath::Step>)];

      // kEqOp
      // kRelOp
      char dummy8[sizeof (blink::xpath::EqTestOp::Opcode)];

      // kMulOp
      char dummy9[sizeof (blink::xpath::NumericOp::Opcode)];

      // kAxisName
      // AxisSpecifier
      char dummy10[sizeof (blink::xpath::Step::Axis)];
    };

    /// The size of the largest semantic type.
    enum { size = sizeof (union_type) };

    /// A buffer to store semantic values.
    union
    {
      /// Strongest alignment constraints.
      long double yyalign_me_;
      /// A buffer large enough to store any of the semantic values.
      char yyraw_[size];
    };
  };

#endif
    /// Backward compatibility (Bison 3.8).
    typedef value_type semantic_type;


    /// Syntax errors thrown from user actions.
    struct syntax_error : std::runtime_error
    {
      syntax_error (const std::string& m)
        : std::runtime_error (m)
      {}

      syntax_error (const syntax_error& s)
        : std::runtime_error (s.what ())
      {}

      ~syntax_error () YY_NOEXCEPT YY_NOTHROW;
    };

    /// Token kinds.
    struct token
    {
      enum token_kind_type
      {
        YYEMPTY = -2,
    YYEOF = 0,                     // "end of file"
    YYerror = 256,                 // error
    YYUNDEF = 257,                 // "invalid token"
    kMulOp = 258,                  // kMulOp
    kEqOp = 259,                   // kEqOp
    kRelOp = 260,                  // kRelOp
    kPlus = 261,                   // kPlus
    kMinus = 262,                  // kMinus
    kOr = 263,                     // kOr
    kAnd = 264,                    // kAnd
    kAxisName = 265,               // kAxisName
    kNodeType = 266,               // kNodeType
    kPI = 267,                     // kPI
    kFunctionName = 268,           // kFunctionName
    kLiteral = 269,                // kLiteral
    kVariableReference = 270,      // kVariableReference
    kNumber = 271,                 // kNumber
    kDotDot = 272,                 // kDotDot
    kSlashSlash = 273,             // kSlashSlash
    kNameTest = 274,               // kNameTest
    kXPathError = 275              // kXPathError
      };
      /// Backward compatibility alias (Bison 3.6).
      typedef token_kind_type yytokentype;
    };

    /// Token kind, as returned by yylex.
    typedef token::token_kind_type token_kind_type;

    /// Backward compatibility alias (Bison 3.6).
    typedef token_kind_type token_type;

    /// Symbol kinds.
    struct symbol_kind
    {
      enum symbol_kind_type
      {
        YYNTOKENS = 30, ///< Number of tokens.
        S_YYEMPTY = -2,
        S_YYEOF = 0,                             // "end of file"
        S_YYerror = 1,                           // error
        S_YYUNDEF = 2,                           // "invalid token"
        S_kMulOp = 3,                            // kMulOp
        S_kEqOp = 4,                             // kEqOp
        S_kRelOp = 5,                            // kRelOp
        S_kPlus = 6,                             // kPlus
        S_kMinus = 7,                            // kMinus
        S_kOr = 8,                               // kOr
        S_kAnd = 9,                              // kAnd
        S_kAxisName = 10,                        // kAxisName
        S_kNodeType = 11,                        // kNodeType
        S_kPI = 12,                              // kPI
        S_kFunctionName = 13,                    // kFunctionName
        S_kLiteral = 14,                         // kLiteral
        S_kVariableReference = 15,               // kVariableReference
        S_kNumber = 16,                          // kNumber
        S_kDotDot = 17,                          // kDotDot
        S_kSlashSlash = 18,                      // kSlashSlash
        S_kNameTest = 19,                        // kNameTest
        S_kXPathError = 20,                      // kXPathError
        S_21_ = 21,                              // '/'
        S_22_ = 22,                              // '@'
        S_23_ = 23,                              // '('
        S_24_ = 24,                              // ')'
        S_25_ = 25,                              // '['
        S_26_ = 26,                              // ']'
        S_27_ = 27,                              // '.'
        S_28_ = 28,                              // ','
        S_29_ = 29,                              // '|'
        S_YYACCEPT = 30,                         // $accept
        S_Expr = 31,                             // Expr
        S_LocationPath = 32,                     // LocationPath
        S_AbsoluteLocationPath = 33,             // AbsoluteLocationPath
        S_RelativeLocationPath = 34,             // RelativeLocationPath
        S_Step = 35,                             // Step
        S_AxisSpecifier = 36,                    // AxisSpecifier
        S_NodeTest = 37,                         // NodeTest
        S_OptionalPredicateList = 38,            // OptionalPredicateList
        S_PredicateList = 39,                    // PredicateList
        S_Predicate = 40,                        // Predicate
        S_DescendantOrSelf = 41,                 // DescendantOrSelf
        S_AbbreviatedStep = 42,                  // AbbreviatedStep
        S_PrimaryExpr = 43,                      // PrimaryExpr
        S_FunctionCall = 44,                     // FunctionCall
        S_ArgumentList = 45,                     // ArgumentList
        S_Argument = 46,                         // Argument
        S_UnionExpr = 47,                        // UnionExpr
        S_PathExpr = 48,                         // PathExpr
        S_FilterExpr = 49,                       // FilterExpr
        S_OrExpr = 50,                           // OrExpr
        S_AndExpr = 51,                          // AndExpr
        S_EqualityExpr = 52,                     // EqualityExpr
        S_RelationalExpr = 53,                   // RelationalExpr
        S_AdditiveExpr = 54,                     // AdditiveExpr
        S_MultiplicativeExpr = 55,               // MultiplicativeExpr
        S_UnaryExpr = 56                         // UnaryExpr
      };
    };

    /// (Internal) symbol kind.
    typedef symbol_kind::symbol_kind_type symbol_kind_type;

    /// The number of tokens.
    static const symbol_kind_type YYNTOKENS = symbol_kind::YYNTOKENS;

    /// A complete symbol.
    ///
    /// Expects its Base type to provide access to the symbol kind
    /// via kind ().
    ///
    /// Provide access to semantic value.
    template <typename Base>
    struct basic_symbol : Base
    {
      /// Alias to Base.
      typedef Base super_type;

      /// Default constructor.
      basic_symbol () YY_NOEXCEPT
        : value ()
      {}

#if 201103L <= YY_CPLUSPLUS
      /// Move constructor.
      basic_symbol (basic_symbol&& that)
        : Base (std::move (that))
        , value ()
      {
        switch (this->kind ())
    {
      case symbol_kind::S_kNodeType: // kNodeType
      case symbol_kind::S_kPI: // kPI
      case symbol_kind::S_kFunctionName: // kFunctionName
      case symbol_kind::S_kLiteral: // kLiteral
      case symbol_kind::S_kVariableReference: // kVariableReference
      case symbol_kind::S_kNumber: // kNumber
      case symbol_kind::S_kNameTest: // kNameTest
        value.move< String > (std::move (that.value));
        break;

      case symbol_kind::S_ArgumentList: // ArgumentList
        value.move< blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Expression>>> > (std::move (that.value));
        break;

      case symbol_kind::S_OptionalPredicateList: // OptionalPredicateList
      case symbol_kind::S_PredicateList: // PredicateList
        value.move< blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Predicate>>> > (std::move (that.value));
        break;

      case symbol_kind::S_Expr: // Expr
      case symbol_kind::S_Predicate: // Predicate
      case symbol_kind::S_PrimaryExpr: // PrimaryExpr
      case symbol_kind::S_FunctionCall: // FunctionCall
      case symbol_kind::S_Argument: // Argument
      case symbol_kind::S_UnionExpr: // UnionExpr
      case symbol_kind::S_PathExpr: // PathExpr
      case symbol_kind::S_FilterExpr: // FilterExpr
      case symbol_kind::S_OrExpr: // OrExpr
      case symbol_kind::S_AndExpr: // AndExpr
      case symbol_kind::S_EqualityExpr: // EqualityExpr
      case symbol_kind::S_RelationalExpr: // RelationalExpr
      case symbol_kind::S_AdditiveExpr: // AdditiveExpr
      case symbol_kind::S_MultiplicativeExpr: // MultiplicativeExpr
      case symbol_kind::S_UnaryExpr: // UnaryExpr
        value.move< blink::Persistent<blink::xpath::Expression> > (std::move (that.value));
        break;

      case symbol_kind::S_LocationPath: // LocationPath
      case symbol_kind::S_AbsoluteLocationPath: // AbsoluteLocationPath
      case symbol_kind::S_RelativeLocationPath: // RelativeLocationPath
        value.move< blink::Persistent<blink::xpath::LocationPath> > (std::move (that.value));
        break;

      case symbol_kind::S_NodeTest: // NodeTest
        value.move< blink::Persistent<blink::xpath::Step::NodeTest> > (std::move (that.value));
        break;

      case symbol_kind::S_Step: // Step
      case symbol_kind::S_DescendantOrSelf: // DescendantOrSelf
      case symbol_kind::S_AbbreviatedStep: // AbbreviatedStep
        value.move< blink::Persistent<blink::xpath::Step> > (std::move (that.value));
        break;

      case symbol_kind::S_kEqOp: // kEqOp
      case symbol_kind::S_kRelOp: // kRelOp
        value.move< blink::xpath::EqTestOp::Opcode > (std::move (that.value));
        break;

      case symbol_kind::S_kMulOp: // kMulOp
        value.move< blink::xpath::NumericOp::Opcode > (std::move (that.value));
        break;

      case symbol_kind::S_kAxisName: // kAxisName
      case symbol_kind::S_AxisSpecifier: // AxisSpecifier
        value.move< blink::xpath::Step::Axis > (std::move (that.value));
        break;

      default:
        break;
    }

      }
#endif

      /// Copy constructor.
      basic_symbol (const basic_symbol& that);

      /// Constructors for typed symbols.
#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t)
        : Base (t)
      {}
#else
      basic_symbol (typename Base::kind_type t)
        : Base (t)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, String&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const String& v)
        : Base (t)
        , value (v)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Expression>>>&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Expression>>>& v)
        : Base (t)
        , value (v)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Predicate>>>&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Predicate>>>& v)
        : Base (t)
        , value (v)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, blink::Persistent<blink::xpath::Expression>&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const blink::Persistent<blink::xpath::Expression>& v)
        : Base (t)
        , value (v)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, blink::Persistent<blink::xpath::LocationPath>&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const blink::Persistent<blink::xpath::LocationPath>& v)
        : Base (t)
        , value (v)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, blink::Persistent<blink::xpath::Step::NodeTest>&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const blink::Persistent<blink::xpath::Step::NodeTest>& v)
        : Base (t)
        , value (v)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, blink::Persistent<blink::xpath::Step>&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const blink::Persistent<blink::xpath::Step>& v)
        : Base (t)
        , value (v)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, blink::xpath::EqTestOp::Opcode&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const blink::xpath::EqTestOp::Opcode& v)
        : Base (t)
        , value (v)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, blink::xpath::NumericOp::Opcode&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const blink::xpath::NumericOp::Opcode& v)
        : Base (t)
        , value (v)
      {}
#endif

#if 201103L <= YY_CPLUSPLUS
      basic_symbol (typename Base::kind_type t, blink::xpath::Step::Axis&& v)
        : Base (t)
        , value (std::move (v))
      {}
#else
      basic_symbol (typename Base::kind_type t, const blink::xpath::Step::Axis& v)
        : Base (t)
        , value (v)
      {}
#endif

      /// Destroy the symbol.
      ~basic_symbol ()
      {
        clear ();
      }



      /// Destroy contents, and record that is empty.
      void clear () YY_NOEXCEPT
      {
        // User destructor.
        symbol_kind_type yykind = this->kind ();
        basic_symbol<Base>& yysym = *this;
        (void) yysym;
        switch (yykind)
        {
       default:
          break;
        }

        // Value type destructor.
switch (yykind)
    {
      case symbol_kind::S_kNodeType: // kNodeType
      case symbol_kind::S_kPI: // kPI
      case symbol_kind::S_kFunctionName: // kFunctionName
      case symbol_kind::S_kLiteral: // kLiteral
      case symbol_kind::S_kVariableReference: // kVariableReference
      case symbol_kind::S_kNumber: // kNumber
      case symbol_kind::S_kNameTest: // kNameTest
        value.template destroy< String > ();
        break;

      case symbol_kind::S_ArgumentList: // ArgumentList
        value.template destroy< blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Expression>>> > ();
        break;

      case symbol_kind::S_OptionalPredicateList: // OptionalPredicateList
      case symbol_kind::S_PredicateList: // PredicateList
        value.template destroy< blink::Persistent<blink::GCedHeapVector<blink::Member<blink::xpath::Predicate>>> > ();
        break;

      case symbol_kind::S_Expr: // Expr
      case symbol_kind::S_Predicate: // Predicate
      case symbol_kind::S_PrimaryExpr: // PrimaryExpr
      case symbol_kind::S_FunctionCall: // FunctionCall
      case symbol_kind::S_Argument: // Argument
      case symbol_kind::S_UnionExpr: // UnionExpr
      case symbol_kind::S_PathExpr: // PathExpr
      case symbol_kind::S_FilterExpr: // FilterExpr
      case symbol_kind::S_OrExpr: // OrExpr
      case symbol_kind::S_AndExpr: // AndExpr
      case symbol_kind::S_EqualityExpr: // EqualityExpr
      case symbol_kind::S_RelationalExpr: // RelationalExpr
      case symbol_kind::S_AdditiveExpr: // AdditiveExpr
      case symbol_kind::S_MultiplicativeExpr: // MultiplicativeExpr
      case symbol_kind::S_UnaryExpr: // UnaryExpr
        value.template destroy< blink::Persistent<blink::xpath::Expression> > ();
        break;

      case symbol_kind::S_LocationPath: // LocationPath
      case symbol_kind::S_AbsoluteLocationPath: // AbsoluteLocationPath
      case symbol_kind::S_RelativeLocationPath: // RelativeLocationPath
        value.template destroy< blink::Persistent<blink::xpath::LocationPath> > ();
        break;

      case symbol_kind::S_NodeTest: // NodeTest
        value.template destroy< blink::Persistent<blink::xpath::Step::NodeTest> > ();
        break;

      case symbol_kind::S_Step: // Step
      case symbol_kind::S_DescendantOrSelf: // DescendantOrSelf
      case symbol_kind::S_AbbreviatedStep: // AbbreviatedStep
        value.template destroy< blink::Persistent<blink::xpath::Step> > ();
        break;

      case symbol_kind::S_kEqOp: // kEqOp
      case symbol_kind::S_kRelOp: // kRelOp
        value.template destroy< blink::xpath::EqTestOp::Opcode > ();
        break;

      case symbol_kind::S_kMulOp: // kMulOp
        value.template destroy< blink::xpath::NumericOp::Opcode > ();
        break;

      case symbol_kind::S_kAxisName: // kAxisName
      case symbol_kind::S_AxisSpecifier: // AxisSpecifier
        value.template destroy< blink::xpath::Step::Axis > ();
        break;

      default:
        break;
    }

        Base::clear ();
      }

#if YYDEBUG || 0
      /// The user-facing name of this symbol.
      const char *name () const YY_NOEXCEPT
      {
        return YyParser::symbol_name (this->kind ());
      }
#endif // #if YYDEBUG || 0


      /// Backward compatibility (Bison 3.6).
      symbol_kind_type type_get () const YY_NOEXCEPT;

      /// Whether empty.
      bool empty () const YY_NOEXCEPT;

      /// Destructive move, \a s is emptied into this.
      void move (basic_symbol& s);

      /// The semantic value.
      value_type value;

    private:
#if YY_CPLUSPLUS < 201103L
      /// Assignment operator.
      basic_symbol& operator= (const basic_symbol& that);
#endif
    };

    /// Type access provider for token (enum) based symbols.
    struct by_kind
    {
      /// The symbol kind as needed by the constructor.
      typedef token_kind_type kind_type;

      /// Default constructor.
      by_kind () YY_NOEXCEPT;

#if 201103L <= YY_CPLUSPLUS
      /// Move constructor.
      by_kind (by_kind&& that) YY_NOEXCEPT;
#endif

      /// Copy constructor.
      by_kind (const by_kind& that) YY_NOEXCEPT;

      /// Constructor from (external) token numbers.
      by_kind (kind_type t) YY_NOEXCEPT;



      /// Record that this symbol is empty.
      void clear () YY_NOEXCEPT;

      /// Steal the symbol kind from \a that.
      void move (by_kind& that);

      /// The (internal) type number (corresponding to \a type).
      /// \a empty when empty.
      symbol_kind_type kind () const YY_NOEXCEPT;

      /// Backward compatibility (Bison 3.6).
      symbol_kind_type type_get () const YY_NOEXCEPT;

      /// The symbol kind.
      /// \a S_YYEMPTY when empty.
      symbol_kind_type kind_;
    };

    /// Backward compatibility for a private implementation detail (Bison 3.6).
    typedef by_kind by_type;

    /// "External" symbols: returned by the scanner.
    struct symbol_type : basic_symbol<by_kind>
    {
      /// Superclass.
      typedef basic_symbol<by_kind> super_type;

      /// Empty symbol.
      symbol_type () YY_NOEXCEPT {}

      /// Constructor for valueless symbols, and symbols from each type.
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok)
        : super_type (token_kind_type (tok))
#else
      symbol_type (int tok)
        : super_type (token_kind_type (tok))
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, String v)
        : super_type (token_kind_type (tok), std::move (v))
#else
      symbol_type (int tok, const String& v)
        : super_type (token_kind_type (tok), v)
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, blink::xpath::EqTestOp::Opcode v)
        : super_type (token_kind_type (tok), std::move (v))
#else
      symbol_type (int tok, const blink::xpath::EqTestOp::Opcode& v)
        : super_type (token_kind_type (tok), v)
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, blink::xpath::NumericOp::Opcode v)
        : super_type (token_kind_type (tok), std::move (v))
#else
      symbol_type (int tok, const blink::xpath::NumericOp::Opcode& v)
        : super_type (token_kind_type (tok), v)
#endif
      {}
#if 201103L <= YY_CPLUSPLUS
      symbol_type (int tok, blink::xpath::Step::Axis v)
        : super_type (token_kind_type (tok), std::move (v))
#else
      symbol_type (int tok, const blink::xpath::Step::Axis& v)
        : super_type (token_kind_type (tok), v)
#endif
      {}
    };

    /// Build a parser object.
    YyParser (blink::xpath::Parser* parser__yyarg);
    virtual ~YyParser ();

#if 201103L <= YY_CPLUSPLUS
    /// Non copyable.
    YyParser (const YyParser&) = delete;
    /// Non copyable.
    YyParser& operator= (const YyParser&) = delete;
#endif

    /// Parse.  An alias for parse ().
    /// \returns  0 iff parsing succeeded.
    int operator() ();

    /// Parse.
    /// \returns  0 iff parsing succeeded.
    virtual int parse ();

#if YYDEBUG
    /// The current debugging stream.
    std::ostream& debug_stream () const YY_ATTRIBUTE_PURE;
    /// Set the current debugging stream.
    void set_debug_stream (std::ostream &);

    /// Type for debugging levels.
    typedef int debug_level_type;
    /// The current debugging level.
    debug_level_type debug_level () const YY_ATTRIBUTE_PURE;
    /// Set the current debugging level.
    void set_debug_level (debug_level_type l);
#endif

    /// Report a syntax error.
    /// \param msg    a description of the syntax error.
    virtual void error (const std::string& msg);

    /// Report a syntax error.
    void error (const syntax_error& err);

#if YYDEBUG || 0
    /// The user-facing name of the symbol whose (internal) number is
    /// YYSYMBOL.  No bounds checking.
    static const char *symbol_name (symbol_kind_type yysymbol);
#endif // #if YYDEBUG || 0


    // Implementation of make_symbol for each token kind.
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_YYEOF ()
      {
        return symbol_type (token::YYEOF);
      }
#else
      static
      symbol_type
      make_YYEOF ()
      {
        return symbol_type (token::YYEOF);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_YYerror ()
      {
        return symbol_type (token::YYerror);
      }
#else
      static
      symbol_type
      make_YYerror ()
      {
        return symbol_type (token::YYerror);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_YYUNDEF ()
      {
        return symbol_type (token::YYUNDEF);
      }
#else
      static
      symbol_type
      make_YYUNDEF ()
      {
        return symbol_type (token::YYUNDEF);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kMulOp (blink::xpath::NumericOp::Opcode v)
      {
        return symbol_type (token::kMulOp, std::move (v));
      }
#else
      static
      symbol_type
      make_kMulOp (const blink::xpath::NumericOp::Opcode& v)
      {
        return symbol_type (token::kMulOp, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kEqOp (blink::xpath::EqTestOp::Opcode v)
      {
        return symbol_type (token::kEqOp, std::move (v));
      }
#else
      static
      symbol_type
      make_kEqOp (const blink::xpath::EqTestOp::Opcode& v)
      {
        return symbol_type (token::kEqOp, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kRelOp (blink::xpath::EqTestOp::Opcode v)
      {
        return symbol_type (token::kRelOp, std::move (v));
      }
#else
      static
      symbol_type
      make_kRelOp (const blink::xpath::EqTestOp::Opcode& v)
      {
        return symbol_type (token::kRelOp, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kPlus ()
      {
        return symbol_type (token::kPlus);
      }
#else
      static
      symbol_type
      make_kPlus ()
      {
        return symbol_type (token::kPlus);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kMinus ()
      {
        return symbol_type (token::kMinus);
      }
#else
      static
      symbol_type
      make_kMinus ()
      {
        return symbol_type (token::kMinus);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kOr ()
      {
        return symbol_type (token::kOr);
      }
#else
      static
      symbol_type
      make_kOr ()
      {
        return symbol_type (token::kOr);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kAnd ()
      {
        return symbol_type (token::kAnd);
      }
#else
      static
      symbol_type
      make_kAnd ()
      {
        return symbol_type (token::kAnd);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kAxisName (blink::xpath::Step::Axis v)
      {
        return symbol_type (token::kAxisName, std::move (v));
      }
#else
      static
      symbol_type
      make_kAxisName (const blink::xpath::Step::Axis& v)
      {
        return symbol_type (token::kAxisName, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kNodeType (String v)
      {
        return symbol_type (token::kNodeType, std::move (v));
      }
#else
      static
      symbol_type
      make_kNodeType (const String& v)
      {
        return symbol_type (token::kNodeType, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kPI (String v)
      {
        return symbol_type (token::kPI, std::move (v));
      }
#else
      static
      symbol_type
      make_kPI (const String& v)
      {
        return symbol_type (token::kPI, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kFunctionName (String v)
      {
        return symbol_type (token::kFunctionName, std::move (v));
      }
#else
      static
      symbol_type
      make_kFunctionName (const String& v)
      {
        return symbol_type (token::kFunctionName, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kLiteral (String v)
      {
        return symbol_type (token::kLiteral, std::move (v));
      }
#else
      static
      symbol_type
      make_kLiteral (const String& v)
      {
        return symbol_type (token::kLiteral, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kVariableReference (String v)
      {
        return symbol_type (token::kVariableReference, std::move (v));
      }
#else
      static
      symbol_type
      make_kVariableReference (const String& v)
      {
        return symbol_type (token::kVariableReference, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kNumber (String v)
      {
        return symbol_type (token::kNumber, std::move (v));
      }
#else
      static
      symbol_type
      make_kNumber (const String& v)
      {
        return symbol_type (token::kNumber, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kDotDot ()
      {
        return symbol_type (token::kDotDot);
      }
#else
      static
      symbol_type
      make_kDotDot ()
      {
        return symbol_type (token::kDotDot);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kSlashSlash ()
      {
        return symbol_type (token::kSlashSlash);
      }
#else
      static
      symbol_type
      make_kSlashSlash ()
      {
        return symbol_type (token::kSlashSlash);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kNameTest (String v)
      {
        return symbol_type (token::kNameTest, std::move (v));
      }
#else
      static
      symbol_type
      make_kNameTest (const String& v)
      {
        return symbol_type (token::kNameTest, v);
      }
#endif
#if 201103L <= YY_CPLUSPLUS
      static
      symbol_type
      make_kXPathError ()
      {
        return symbol_type (token::kXPathError);
      }
#else
      static
      symbol_type
      make_kXPathError ()
      {
        return symbol_type (token::kXPathError);
      }
#endif


  private:
#if YY_CPLUSPLUS < 201103L
    /// Non copyable.
    YyParser (const YyParser&);
    /// Non copyable.
    YyParser& operator= (const YyParser&);
#endif


    /// Stored state numbers (used for stacks).
    typedef signed char state_type;

    /// Compute post-reduction state.
    /// \param yystate   the current state
    /// \param yysym     the nonterminal to push on the stack
    static state_type yy_lr_goto_state_ (state_type yystate, int yysym);

    /// Whether the given \c yypact_ value indicates a defaulted state.
    /// \param yyvalue   the value to check
    static bool yy_pact_value_is_default_ (int yyvalue) YY_NOEXCEPT;

    /// Whether the given \c yytable_ value indicates a syntax error.
    /// \param yyvalue   the value to check
    static bool yy_table_value_is_error_ (int yyvalue) YY_NOEXCEPT;

    static const signed char yypact_ninf_;
    static const signed char yytable_ninf_;

    /// Convert a scanner token kind \a t to a symbol kind.
    /// In theory \a t should be a token_kind_type, but character literals
    /// are valid, yet not members of the token_kind_type enum.
    static symbol_kind_type yytranslate_ (int t) YY_NOEXCEPT;

#if YYDEBUG || 0
    /// For a symbol, its name in clear.
    static const char* const yytname_[];
#endif // #if YYDEBUG || 0


    // Tables.
    // YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
    // STATE-NUM.
    static const signed char yypact_[];

    // YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
    // Performed when YYTABLE does not specify something else to do.  Zero
    // means the default is an error.
    static const signed char yydefact_[];

    // YYPGOTO[NTERM-NUM].
    static const signed char yypgoto_[];

    // YYDEFGOTO[NTERM-NUM].
    static const signed char yydefgoto_[];

    // YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
    // positive, shift that token.  If negative, reduce the rule whose
    // number is the opposite.  If YYTABLE_NINF, syntax error.
    static const signed char yytable_[];

    static const signed char yycheck_[];

    // YYSTOS[STATE-NUM] -- The symbol kind of the accessing symbol of
    // state STATE-NUM.
    static const signed char yystos_[];

    // YYR1[RULE-NUM] -- Symbol kind of the left-hand side of rule RULE-NUM.
    static const signed char yyr1_[];

    // YYR2[RULE-NUM] -- Number of symbols on the right-hand side of rule RULE-NUM.
    static const signed char yyr2_[];


#if YYDEBUG
    // YYRLINE[YYN] -- Source line where rule number YYN was defined.
    static const short yyrline_[];
    /// Report on the debug stream that the rule \a r is going to be reduced.
    virtual void yy_reduce_print_ (int r) const;
    /// Print the state stack on the debug stream.
    virtual void yy_stack_print_ () const;

    /// Debugging level.
    int yydebug_;
    /// Debug stream.
    std::ostream* yycdebug_;

    /// \brief Display a symbol kind, value and location.
    /// \param yyo    The output stream.
    /// \param yysym  The symbol.
    template <typename Base>
    void yy_print_ (std::ostream& yyo, const basic_symbol<Base>& yysym) const;
#endif

    /// \brief Reclaim the memory associated to a symbol.
    /// \param yymsg     Why this token is reclaimed.
    ///                  If null, print nothing.
    /// \param yysym     The symbol.
    template <typename Base>
    void yy_destroy_ (const char* yymsg, basic_symbol<Base>& yysym) const;

  private:
    /// Type access provider for state based symbols.
    struct by_state
    {
      /// Default constructor.
      by_state () YY_NOEXCEPT;

      /// The symbol kind as needed by the constructor.
      typedef state_type kind_type;

      /// Constructor.
      by_state (kind_type s) YY_NOEXCEPT;

      /// Copy constructor.
      by_state (const by_state& that) YY_NOEXCEPT;

      /// Record that this symbol is empty.
      void clear () YY_NOEXCEPT;

      /// Steal the symbol kind from \a that.
      void move (by_state& that);

      /// The symbol kind (corresponding to \a state).
      /// \a symbol_kind::S_YYEMPTY when empty.
      symbol_kind_type kind () const YY_NOEXCEPT;

      /// The state number used to denote an empty symbol.
      /// We use the initial state, as it does not have a value.
      enum { empty_state = 0 };

      /// The state.
      /// \a empty when empty.
      state_type state;
    };

    /// "Internal" symbol: element of the stack.
    struct stack_symbol_type : basic_symbol<by_state>
    {
      /// Superclass.
      typedef basic_symbol<by_state> super_type;
      /// Construct an empty symbol.
      stack_symbol_type ();
      /// Move or copy construction.
      stack_symbol_type (YY_RVREF (stack_symbol_type) that);
      /// Steal the contents from \a sym to build this.
      stack_symbol_type (state_type s, YY_MOVE_REF (symbol_type) sym);
#if YY_CPLUSPLUS < 201103L
      /// Assignment, needed by push_back by some old implementations.
      /// Moves the contents of that.
      stack_symbol_type& operator= (stack_symbol_type& that);

      /// Assignment, needed by push_back by other implementations.
      /// Needed by some other old implementations.
      stack_symbol_type& operator= (const stack_symbol_type& that);
#endif
    };

    /// A stack with random access from its top.
    template <typename T, typename S = std::vector<T> >
    class stack
    {
    public:
      // Hide our reversed order.
      typedef typename S::iterator iterator;
      typedef typename S::const_iterator const_iterator;
      typedef typename S::size_type size_type;
      typedef typename std::ptrdiff_t index_type;

      stack (size_type n = 200) YY_NOEXCEPT
        : seq_ (n)
      {}

#if 201103L <= YY_CPLUSPLUS
      /// Non copyable.
      stack (const stack&) = delete;
      /// Non copyable.
      stack& operator= (const stack&) = delete;
#endif

      /// Random access.
      ///
      /// Index 0 returns the topmost element.
      const T&
      operator[] (index_type i) const
      {
        return seq_[size_type (size () - 1 - i)];
      }

      /// Random access.
      ///
      /// Index 0 returns the topmost element.
      T&
      operator[] (index_type i)
      {
        return seq_[size_type (size () - 1 - i)];
      }

      /// Steal the contents of \a t.
      ///
      /// Close to move-semantics.
      void
      push (YY_MOVE_REF (T) t)
      {
        seq_.push_back (T ());
        operator[] (0).move (t);
      }

      /// Pop elements from the stack.
      void
      pop (std::ptrdiff_t n = 1) YY_NOEXCEPT
      {
        for (; 0 < n; --n)
          seq_.pop_back ();
      }

      /// Pop all elements from the stack.
      void
      clear () YY_NOEXCEPT
      {
        seq_.clear ();
      }

      /// Number of elements on the stack.
      index_type
      size () const YY_NOEXCEPT
      {
        return index_type (seq_.size ());
      }

      /// Iterator on top of the stack (going downwards).
      const_iterator
      begin () const YY_NOEXCEPT
      {
        return seq_.begin ();
      }

      /// Bottom of the stack.
      const_iterator
      end () const YY_NOEXCEPT
      {
        return seq_.end ();
      }

      /// Present a slice of the top of a stack.
      class slice
      {
      public:
        slice (const stack& stack, index_type range) YY_NOEXCEPT
          : stack_ (stack)
          , range_ (range)
        {}

        const T&
        operator[] (index_type i) const
        {
          return stack_[range_ - i];
        }

      private:
        const stack& stack_;
        index_type range_;
      };

    private:
#if YY_CPLUSPLUS < 201103L
      /// Non copyable.
      stack (const stack&);
      /// Non copyable.
      stack& operator= (const stack&);
#endif
      /// The wrapped container.
      S seq_;
    };


    /// Stack type.
    typedef stack<stack_symbol_type> stack_type;

    /// The stack.
    stack_type yystack_;

    /// Push a new state on the stack.
    /// \param m    a debug message to display
    ///             if null, no trace is output.
    /// \param sym  the symbol
    /// \warning the contents of \a s.value is stolen.
    void yypush_ (const char* m, YY_MOVE_REF (stack_symbol_type) sym);

    /// Push a new look ahead token on the state on the stack.
    /// \param m    a debug message to display
    ///             if null, no trace is output.
    /// \param s    the state
    /// \param sym  the symbol (for its value and location).
    /// \warning the contents of \a sym.value is stolen.
    void yypush_ (const char* m, state_type s, YY_MOVE_REF (symbol_type) sym);

    /// Pop \a n symbols from the stack.
    void yypop_ (int n = 1) YY_NOEXCEPT;

    /// Constants.
    enum
    {
      yylast_ = 122,     ///< Last index in yytable_.
      yynnts_ = 27,  ///< Number of nonterminal symbols.
      yyfinal_ = 47 ///< Termination state number.
    };


    // User arguments.
    blink::xpath::Parser* parser_;

  };


#line 76 "third_party/blink/renderer/core/xml/xpath_grammar.y"
} // xpathyy
#line 1723 "third_party/blink/renderer/core/xml/xpath_grammar_generated.h"




#endif // !YY_YY_THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_GRAMMAR_GENERATED_HH_INCLUDED
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_GRAMMAR_GENERATED_H_
