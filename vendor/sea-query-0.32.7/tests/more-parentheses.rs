use sea_query::{tests_cfg::Glyph, Cond, Expr, MysqlQueryBuilder, Query};

#[test]
fn test_more_parentheses() {
    let query = Query::select()
        .column(Glyph::Image)
        .from(Glyph::Table)
        .cond_where(Cond::all())
        .cond_where(Expr::val(1).eq(1))
        .cond_where(Expr::val(2).eq(2))
        .cond_where(Cond::any().add(Expr::val(3).eq(3)).add(Expr::val(4).eq(4)))
        .to_owned();

    assert_eq!(
        query.to_string(MysqlQueryBuilder),
        "SELECT `image` FROM `glyph` WHERE (1 = 1) AND (2 = 2) AND ((3 = 3) OR (4 = 4))"
    );
}

#[test]
fn test_more_parentheses_complex() {
    // Add pagination
    let mut pagination = Cond::all();
    let lt_value = Expr::col(Glyph::Aspect)
        .lt(1)
        .or(Expr::col(Glyph::Aspect).is_null());
    let lt_id = Expr::col(Glyph::Aspect)
        .is(1)
        .and(Expr::col(Glyph::Id).lt(10));
    pagination = pagination.add(lt_value.or(lt_id));

    // Add filtering
    let mut all = Cond::all();
    all = all.add(Expr::col(Glyph::Image).eq("png"));

    let mut nested = Cond::all();
    nested = nested.add(Expr::col(Glyph::Table).gte(5));
    nested = nested.add(Expr::col(Glyph::Tokens).lte(3));
    all = all.add(nested);

    let mut any = Cond::any();
    any = any.add(Expr::col(Glyph::Image).like("%.jpg"));
    any = any.add(all);
    let filtering = any;

    // Query
    let query = Query::select()
        .column(Glyph::Id)
        .from(Glyph::Table)
        .cond_where(Cond::all())
        .cond_where(pagination)
        .cond_where(filtering)
        .to_owned();

    assert_eq!(
        query.to_string(MysqlQueryBuilder),
        "SELECT `id` FROM `glyph` WHERE ((`aspect` < 1) OR (`aspect` IS NULL) OR ((`aspect` IS 1) AND (`id` < 10))) AND ((`image` LIKE '%.jpg') OR ((`image` = 'png') AND ((`glyph` >= 5) AND (`tokens` <= 3))))"
    );
}
