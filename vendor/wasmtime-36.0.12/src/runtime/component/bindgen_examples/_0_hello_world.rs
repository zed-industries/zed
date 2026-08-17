bindgen!({
  inline: r#"
      package my:project;
      world hello-world {
          import name: func() -> string;
          export greet: func();
      }
  "#,
});
