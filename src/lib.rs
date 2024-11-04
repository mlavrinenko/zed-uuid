use uuid::Uuid;
use zed_extension_api::{
    Extension, SlashCommand, SlashCommandOutput, SlashCommandOutputSection, Worktree,
};

pub struct UuidExtension {}
zed_extension_api::register_extension!(UuidExtension);

impl Extension for UuidExtension {
    fn new() -> Self {
        Self {}
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "uuid" => {
                let uuid = Uuid::new_v4().to_string();
                let text = format!("{uuid}");

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..uuid.len()).into(),
                        label: "UUID".to_string(),
                    }],
                    text,
                })
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::*;
    use zed_extension_api::SlashCommand;

    #[test]
    fn test_uuid_slash_command() {
        let result = UuidExtension::new().run_slash_command(
            SlashCommand {
                name: "uuid".to_string(),
                description: "".to_string(),
                requires_argument: false,
                tooltip_text: "".to_string(),
            },
            vec![],
            None,
        );

        assert!(result.is_ok());
        assert!(Uuid::from_str(&result.unwrap().text).is_ok());
    }

    #[test]
    fn test_unknown_slash_command() {
        let result = UuidExtension::new().run_slash_command(
            SlashCommand {
                name: "unknown".to_string(),
                description: "".to_string(),
                requires_argument: false,
                tooltip_text: "".to_string(),
            },
            vec![],
            None,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "unknown slash command: \"unknown\"");
    }
}
