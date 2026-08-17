 // A lightweight class, with pointer-like methods, that contains an int
struct X
{
    int i_;

    constexpr const int &operator*() const { return i_; }
    constexpr int &operator*() { return i_; }
    constexpr const int *get() const { return &i_; }
    constexpr int *get() { return &i_; }
    constexpr const int *operator->() const { return &i_; }
    constexpr int *operator->() { return &i_; }

    constexpr X(int i) : i_(i) {}
};

struct XWithImplicitIntStarConversion
{
    int i_;

    constexpr const int &operator*() const { return i_; }
    constexpr int &operator*() { return i_; }
    constexpr const int *get() const { return &i_; }
    constexpr int *get() { return &i_; }
    constexpr const int *operator->() const { return &i_; }
    constexpr int *operator->() { return &i_; }
    constexpr operator int* () { return &i_; }

    constexpr XWithImplicitIntStarConversion(int i) : i_(i) {}
};

struct XWithImplicitConstIntStarConversion
{
    int i_;

    constexpr const int &operator*() const { return i_; }
    constexpr int &operator*() { return i_; }
    constexpr const int *get() const { return &i_; }
    constexpr int *get() { return &i_; }
    constexpr const int *operator->() const { return &i_; }
    constexpr int *operator->() { return &i_; }
    constexpr operator const int* () const { return &i_; }

    constexpr XWithImplicitConstIntStarConversion(int i) : i_(i) {}
};

struct ExplicitX
{
    int i_;

    constexpr const int &operator*() const { return i_; }
    constexpr int &operator*() { return i_; }
    constexpr const int *get() const { return &i_; }
    constexpr int *get() { return &i_; }
    constexpr const int *operator->() const { return &i_; }
    constexpr int *operator->() { return &i_; }

    constexpr explicit ExplicitX(int i) : i_(i) {}
};

struct MoveConstructibleFromX
{
    int i_;

    constexpr const int &operator*() const { return i_; }
    constexpr int &operator*() { return i_; }
    constexpr const int *get() const { return &i_; }
    constexpr int *get() { return &i_; }
    constexpr const int *operator->() const { return &i_; }
    constexpr int *operator->() { return &i_; }

    constexpr MoveConstructibleFromX(int i) : i_(i) {}
    constexpr MoveConstructibleFromX(X&& x) : i_(x.i_) {}
};

struct ExplicitMoveConstructibleFromX
{
    int i_;

    constexpr const int &operator*() const { return i_; }
    constexpr int &operator*() { return i_; }
    constexpr const int *get() const { return &i_; }
    constexpr int *get() { return &i_; }
    constexpr const int *operator->() const { return &i_; }
    constexpr int *operator->() { return &i_; }

    constexpr ExplicitMoveConstructibleFromX(int i) : i_(i) {}
    constexpr explicit ExplicitMoveConstructibleFromX(X&& x) : i_(x.i_) {}
};

struct CopyConstructibleFromX
{
    int i_;

    constexpr const int &operator*() const { return i_; }
    constexpr int &operator*() { return i_; }
    constexpr const int *get() const { return &i_; }
    constexpr int *get() { return &i_; }
    constexpr const int *operator->() const { return &i_; }
    constexpr int *operator->() { return &i_; }

    constexpr CopyConstructibleFromX(int i) : i_(i) {}
    constexpr CopyConstructibleFromX(const X& x) : i_(x.i_) {}
};

struct ExplicitCopyConstructibleFromX
{
    int i_;

    constexpr const int &operator*() const { return i_; }
    constexpr int &operator*() { return i_; }
    constexpr const int *get() const { return &i_; }
    constexpr int *get() { return &i_; }
    constexpr const int *operator->() const { return &i_; }
    constexpr int *operator->() { return &i_; }

    constexpr ExplicitCopyConstructibleFromX(int i) : i_(i) {}
    constexpr explicit ExplicitCopyConstructibleFromX(const X& x) : i_(x.i_) {}
};
