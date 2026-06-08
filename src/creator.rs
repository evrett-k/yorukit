use dialoguer::{Input, Select, MultiSelect, Confirm, theme::ColorfulTheme};
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

    let darwins = vec![
        "ios",
        "macos",
        "tvos",
        "watchos",
        "visionos",
        "bridgeos <experimental>",
        "audioos <experimental>",
    ];

    let templates = vec![
        "tweak_c",
        "application_swift"
    ];

    // darwin
    let darwin_selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose OS(es)")
        .items(&darwins)
        .interact()
        .unwrap();

    let chosen_darwins: Vec<&str> = darwin_selection.iter().map(|&i| darwins[i]).collect();

    // arch filtering
    let arch_display = vec![
        ("arm64", "arm64"),
        ("arm64e", "arm64e"),
        ("arm64_32", "arm64_32"),
        ("arm7k", "arm (armv7k)"),
        ("arm7s", "arm (armv7s)"),
        ("arm", "arm (armv7)"),
        ("x86_64", "x86_64"),
    ];

    let os_arch_options: Vec<((&str, &str), String)> = darwins
        .iter()
        .filter(|&&os| chosen_darwins.contains(&os))
        .flat_map(|&os| {
            let prefix = match os {
                "ios" => "iphoneos",
                "macos" => "darwin",
                "tvos" => "appletvos",
                "watchos" => "watchos",
                "visionos" => "xros",
                "bridgeos <experimental>" => "bridgeos",
                "audioos <experimental>" => "audioos",
                _ => os,
            };
            arch_display.iter().filter(|(id, _)| {
                match *id {
                    "arm" | "arm7s" => os.contains("ios"),
                    "arm7k" | "arm64_32" => os.contains("watchos"),
                    "arm64" => true,
                    "arm64e" => os.contains("ios") || os.contains("macos") || os.contains("tvos") || os.contains("visionos"),
                    "x86_64" => os.contains("macos"),
                    _ => false,
                }
            }).map(move |(id, _)| ((*id, prefix), format!("{}-{}", prefix, id)))
        }).collect();

    let combo_display: Vec<&str> = os_arch_options.iter().map(|(_, s)| s.as_str()).collect();

    let arch_selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose architecture(s)")
        .items(&combo_display)
        .interact()
        .unwrap();

    let chosen_arch_strings: Vec<String> = arch_selection.iter()
        .map(|&i| os_arch_options[i].1.clone())
        .collect();

    // template
    let template_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a template")
        .default(0)
        .items(&templates)
        .interact()
        .unwrap();

    let chosen_template = templates[template_selection];

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

    // xcodegen
    let xcodegen: bool = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to generate an Xcode project?")
        .interact()
        .unwrap();

    match std::fs::create_dir(&project_name) {
        Ok(_) => {
            println!("Created {}", project_name);

            // xcodegen logic DO_LATER

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
                    architectures: chosen_arch_strings,
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