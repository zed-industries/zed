use bon::Builder;

#[derive(Builder)]
struct IncorrectOrder1 {
    #[builder(start_fn)]
    _a: (),
    _b: (),
    #[builder(start_fn)]
    _c: (),
}

#[derive(Builder)]
struct IncorrectOrder2 {
    #[builder(finish_fn)]
    _a: (),
    _b: (),
    #[builder(start_fn)]
    _c: (),
}

#[derive(Builder)]
struct IncorrectOrder3 {
    _a: (),
    #[builder(start_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder4 {
    _a: (),
    #[builder(finish_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder5 {
    #[builder(skip)]
    _a: (),
    #[builder(start_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder6 {
    #[builder(skip)]
    _a: (),
    #[builder(finish_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder7 {
    #[builder(finish_fn)]
    _a: (),
    #[builder(start_fn)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder8 {
    #[builder(start_fn)]
    _a: (),
    #[builder(finish_fn)]
    _b: (),
    #[builder(start_fn)]
    _c: (),
}

#[derive(Builder)]
struct IncorrectOrder9 {
    _a: (),
    #[builder(field)]
    _b: (),
}

#[derive(Builder)]
struct IncorrectOrder10 {
    #[builder(start_fn)]
    _a: (),

    _b: (),

    #[builder(field)]
    _c: (),
}

#[derive(Builder)]
struct IncorrectOrder11 {
    #[builder(finish_fn)]
    _a: (),

    #[builder(field)]
    _b: (),
}


#[derive(Builder)]
struct IncorrectOrder12 {
    #[builder(skip)]
    _a: (),

    #[builder(field)]
    _b: (),
}


#[derive(Builder)]
struct IncorrectOrder13 {
    #[builder(field)]
    _a: (),
    _b: (),
    #[builder(field)]
    _c: (),
}



fn main() {}
