test_issue_1063 = executable('issue-1063',
  files([ 'issue-1063/main.c' ]),
  dependencies: freetype_dep,
)

test_env = ['FREETYPE_TESTS_DATA_DIR='
            + join_paths(meson.current_source_dir(), 'data')]

test('issue-1063',
  test_issue_1063,
  env: test_env,
  suite: 'regression')

# EOF
