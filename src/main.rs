use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Actor {
    name: String,
    sprites: HashMap<String, String>,
}
impl Default for Actor {
    fn default() -> Self {
        Self {
            name: String::new(),
            sprites: HashMap::new(),
        }
    }
}

fn parse_decls(lines: &Vec<&str>) -> Result<HashMap<String, Actor>, String> {
    let mut actor_pool = HashMap::new();
    let mut current_actor_id: &str = "";
    let mut is_parsing_spritemap = false;
    let mut spritemap_string = String::new();

    for line in lines {
        if let Some('[') = line.chars().nth(0) {
            if !line.ends_with("]") {
                return Err("Invalid [actor]. Double check your script".into());
            }

            let actor_id = &line[1..line.len() - 1];
            if actor_pool.contains_key(actor_id) {
                return Err(format!(
                    "Actor identified as {actor_id} is defined more than once. Double check your script file."
                ));
            }
            current_actor_id = actor_id;
            actor_pool.insert(actor_id.to_owned(), Actor::default());
        } else {
            if !is_parsing_spritemap {
                let equal_sign_idx = line.find("=").ok_or("Cannot find equal sign")?;
                let property_name = line[0..equal_sign_idx].trim();
                if property_name.contains(" ") {
                    return Err(
                    "Whitespace detected in actor's property name. Double check your script file."
                        .into(),
                );
                }

                if let Some(actor) = actor_pool.get_mut(current_actor_id) {
                    match property_name {
                        "name" => actor.name = line[equal_sign_idx + 1..line.len()].trim().into(),
                        "sprites" => {
                            spritemap_string.push_str(&line[equal_sign_idx + 1..line.len()]);
                            is_parsing_spritemap = true;
                        }
                        _ => {
                            return Err(format!(
                                "what kind of actor property is ts 💔: {property_name}"
                            ));
                        }
                    }
                }
            } else {
                spritemap_string.push_str(&line);
                // the spritemap ended
                if line.trim() == "}" {
                    let trimmed_spritemap_string: String = spritemap_string
                        .trim()
                        .chars()
                        .filter(|ch| *ch != ' ' && *ch != '{' && *ch != '}')
                        .collect();
                    let key_values: Vec<&str> = trimmed_spritemap_string.split(",").collect();
                    assert!(key_values[0].starts_with("init:"));
                    for kv_pair_string in key_values {
                        let colon_idx = kv_pair_string
                            .find(':')
                            .expect("Could not find comma ':' while parsing map");
                        let (key, value_with_comma) = kv_pair_string.split_at(colon_idx);
                        let value = &value_with_comma[1..];
                        dbg!(key);
                        dbg!(value);

                        if let Some(actor) = actor_pool.get_mut(current_actor_id) {
                            actor
                                .sprites
                                .insert(key.into(), (value[1..value.len() - 1]).to_owned());
                        }
                    }
                }
            }
        }
    }
    Ok(actor_pool)
}

fn main() {
    let data = fs::read_to_string("script.vns").expect("Could not read script file");

    let lines: Vec<&str> = data.lines().filter(|x| !x.is_empty()).collect();

    let mut decl_lines: Vec<&str> = Vec::new();
    let mut script_lines: Vec<&str> = Vec::new();
    let mut is_in_decls = false;
    let mut is_in_scripts = false;

    for line in lines {
        is_in_decls = match line {
            "#decls" => true,
            "#enddecls" => false,
            _ => is_in_decls,
        };

        is_in_scripts = match line {
            "#script" => true,
            "#endscript" => false,
            _ => is_in_scripts,
        };

        if is_in_decls && line != "#decls" {
            decl_lines.push(line);
        }
        if is_in_scripts && line != "#script" {
            script_lines.push(line);
        }
    }

    // dbg!(&decl_lines);
    // dbg!(&script_lines);

    let actor_pool = parse_decls(&decl_lines).unwrap();
    dbg!(actor_pool);
}
