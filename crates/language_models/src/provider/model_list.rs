pub(crate) fn merge_models_by_name<T, I, F>(
    discovered_models: impl IntoIterator<Item = T>,
    configured_models: I,
    name: F,
) -> Vec<T>
where
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> &str,
{
    let mut models = discovered_models.into_iter().collect::<Vec<_>>();

    for configured_model in configured_models {
        if let Some(position) = models
            .iter()
            .position(|model| name(model) == name(&configured_model))
        {
            models[position] = configured_model;
        } else {
            models.push(configured_model);
        }
    }

    models
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestModel {
        name: &'static str,
        value: u32,
    }

    fn merge(
        discovered_models: Vec<TestModel>,
        configured_models: Vec<TestModel>,
    ) -> Vec<TestModel> {
        merge_models_by_name(discovered_models, configured_models, |model| model.name)
    }

    #[test]
    fn test_no_configured_models() {
        let discovered_models = vec![
            TestModel {
                name: "alpha",
                value: 1,
            },
            TestModel {
                name: "beta",
                value: 2,
            },
        ];

        assert_eq!(
            merge(discovered_models.clone(), Vec::new()),
            discovered_models
        );
    }

    #[test]
    fn test_no_discovered_models() {
        let configured_models = vec![
            TestModel {
                name: "alpha",
                value: 1,
            },
            TestModel {
                name: "beta",
                value: 2,
            },
        ];

        assert_eq!(
            merge(Vec::new(), configured_models.clone()),
            configured_models
        );
    }

    #[test]
    fn test_exact_name_replaces() {
        let models = merge(
            vec![
                TestModel {
                    name: "alpha",
                    value: 1,
                },
                TestModel {
                    name: "beta",
                    value: 2,
                },
            ],
            vec![TestModel {
                name: "alpha",
                value: 3,
            }],
        );

        assert_eq!(
            models,
            vec![
                TestModel {
                    name: "alpha",
                    value: 3,
                },
                TestModel {
                    name: "beta",
                    value: 2,
                },
            ]
        );
    }

    #[test]
    fn test_configured_only_appended() {
        let models = merge(
            vec![TestModel {
                name: "alpha",
                value: 1,
            }],
            vec![TestModel {
                name: "beta",
                value: 2,
            }],
        );

        assert_eq!(
            models,
            vec![
                TestModel {
                    name: "alpha",
                    value: 1,
                },
                TestModel {
                    name: "beta",
                    value: 2,
                },
            ]
        );
    }

    #[test]
    fn test_similar_names_not_matched() {
        let models = merge(
            vec![TestModel {
                name: "qwen2.5-coder:1.5b",
                value: 1,
            }],
            vec![TestModel {
                name: "qwen2.5-coder:3b",
                value: 2,
            }],
        );

        assert_eq!(
            models,
            vec![
                TestModel {
                    name: "qwen2.5-coder:1.5b",
                    value: 1,
                },
                TestModel {
                    name: "qwen2.5-coder:3b",
                    value: 2,
                },
            ]
        );
    }

    #[test]
    fn test_discovered_order_preserved() {
        let models = merge(
            vec![
                TestModel {
                    name: "zeta",
                    value: 1,
                },
                TestModel {
                    name: "alpha",
                    value: 2,
                },
            ],
            vec![TestModel {
                name: "zeta",
                value: 3,
            }],
        );

        assert_eq!(
            models,
            vec![
                TestModel {
                    name: "zeta",
                    value: 3,
                },
                TestModel {
                    name: "alpha",
                    value: 2,
                },
            ]
        );
    }

    #[test]
    fn test_duplicate_configured_last_wins() {
        let models = merge(
            vec![TestModel {
                name: "alpha",
                value: 1,
            }],
            vec![
                TestModel {
                    name: "alpha",
                    value: 2,
                },
                TestModel {
                    name: "alpha",
                    value: 3,
                },
            ],
        );

        assert_eq!(
            models,
            vec![TestModel {
                name: "alpha",
                value: 3,
            }]
        );
    }
}
