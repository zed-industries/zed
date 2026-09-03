; Tag the model's first construct so a run gutter appears at the top of the
; file; tasks tagged `dbt-model` (dbt run/test/build/compile/show --select
; $ZED_STEM) bind to it.
(template
  .
  (_) @run
  (#set! tag dbt-model))
