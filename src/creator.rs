use dialoguer::{Input, Select, console::TermFamily::File, theme::ColorfulTheme};
use serde::Serialize;

#[derive(Serialize)]
struct ProjectConfig {
    name: String,
    id: String,
    version: String,
    author: String,
}

#[derive(Serialize)]
struct TargetConfig {
    mode: String,
    min_os: String,
    architectures: Vec<String>,
}

#[derive(Serialize)]
struct SigningConfig {
    entitlements: String,
    mode: String
}

#[derive(Serialize)]
struct YoruKitToml {
    project: ProjectConfig,
    target: TargetConfig,
    signing: SigningConfig,
}

pub fn creator() {
    use std::result::Result::{Ok, Err};

    println!("YoruKit Project Creator");
    println!("-----------------------");

    let os = vec![
        "ios",
        "macos",
        "tvos",
        "watchos",
        "visionos",
        "bridgeos <experimental>",
        "audioos <experimental>",
    ];

    let templates = vec![
        "tweak_objc",
        "application_objc"
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a template")
        .default(0)
        .items(&templates)
        .interact()
        .unwrap();

    let chosen_template = templates[selection];

    // project
    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Name")
        .interact_text()
        .unwrap();

    // package
    let default_package = format!("com.yourcompany.{}", project_name.to_lowercase());
    let package_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Package Name")
        .interact_text()
        .unwrap();

    // author
    let author: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author/Maintainer Name")
        .default("DO_LATER".to_string())
        .interact_text()
        .unwrap();

    match std::fs::create_dir(&project_name) {
        Ok(_) => {
            println!("Created {}", project_name);

            let config_payload = YoruKitToml {
                project: ProjectConfig {
                    name: project_name.clone(),
                    id: package_name,
                    version: "1.0.0".to_string(),
                    author: author,
                },
                target: TargetConfig {
                    mode: "rootless".to_string(),
                    min_os: "15.0".to_string(),
                    architectures: vec!["arm64".to_string(), "arm64e".to_string()],
                },
                signing: SigningConfig {
                    entitlements: "entitlements.plist".to_string(),
                    mode: "none".to_string(),
                },
            };

            match toml::to_string_pretty(&config_payload) {
                Ok(toml_string) => {
                    let file_path = format!("{}/yoru.toml", project_name);
                    match std::fs::write(&file_path, toml_string) {
                        Ok(_) => println!("Wrote yoru.toml at ./{}", file_path),
                        Err(e) => eprint!("Error: Failed to write yoru.toml : {}", e),
                    }
                }
                Err(e) => eprint!("Error: Failed to format config to TOML: {}", e),
            }
            let plist_path = format!("{}/entitlements.plist", project_name);
            let default_plist_content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n\n</dict>\n</plist>";

            std::fs::write(&plist_path, default_plist_content)
                .expect("Error: Failed to create entitlements.plist");
                
            println!("Wrote entitlements.plist at ./{}", plist_path);
        }
        Err(e) => {
            eprintln!("Could not create directory {} : {}", project_name, e);
        }
    };
}