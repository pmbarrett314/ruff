pub(crate) mod rules;
pub mod settings;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use rustc_hash::FxHashMap;

    use super::settings::{BannedApi, Strictness};
    use crate::linter::test_path;
    use crate::registry::RuleCode;
    use crate::settings::Settings;

    #[test]
    fn ban_parent_imports() -> Result<()> {
        let diagnostics = test_path(
            Path::new("./resources/test/fixtures/flake8_tidy_imports/TID252.py"),
            &Settings {
                flake8_tidy_imports: super::settings::Settings {
                    ban_relative_imports: Strictness::Parents,
                    ..Default::default()
                },
                ..Settings::for_rules(vec![RuleCode::TID252])
            },
        )?;
        insta::assert_yaml_snapshot!(diagnostics);
        Ok(())
    }

    #[test]
    fn ban_all_imports() -> Result<()> {
        let diagnostics = test_path(
            Path::new("./resources/test/fixtures/flake8_tidy_imports/TID252.py"),
            &Settings {
                flake8_tidy_imports: super::settings::Settings {
                    ban_relative_imports: Strictness::All,
                    ..Default::default()
                },
                ..Settings::for_rules(vec![RuleCode::TID252])
            },
        )?;
        insta::assert_yaml_snapshot!(diagnostics);
        Ok(())
    }

    #[test]
    fn banned_api_true_positives() -> Result<()> {
        let diagnostics = test_path(
            Path::new("./resources/test/fixtures/flake8_tidy_imports/TID251.py"),
            &Settings {
                flake8_tidy_imports: super::settings::Settings {
                    banned_api: FxHashMap::from_iter([
                        (
                            "cgi".to_string(),
                            BannedApi {
                                msg: "The cgi module is deprecated.".to_string(),
                            },
                        ),
                        (
                            "typing.TypedDict".to_string(),
                            BannedApi {
                                msg: "Use typing_extensions.TypedDict instead.".to_string(),
                            },
                        ),
                    ])
                    .into(),
                    ..Default::default()
                },
                ..Settings::for_rules(vec![RuleCode::TID251])
            },
        )?;
        insta::assert_yaml_snapshot!(diagnostics);
        Ok(())
    }
}
