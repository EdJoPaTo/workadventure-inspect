use serde::Deserialize;
use serde_json::Value;
use url::Url;

mod cli;

#[derive(Debug, Deserialize)]
pub struct Map {
    layers: Vec<Layer>,
    version: String,
    width: u32,
    height: u32,
    infinite: bool,
    nextlayerid: u32,
    nextobjectid: u32,
    renderorder: String,
    tileheight: u32,
    tilesets: Vec<Tileset>,
}

#[derive(Debug, Deserialize)]
pub struct Layer {
    // height: u32,
    // width: u32,
    id: u32,
    name: String,
    opacity: f32,
    visible: bool,
    x: u32,
    y: u32,

    #[serde(default)]
    properties: Vec<LayerProperty>,
}

#[derive(Debug, Deserialize)]
pub struct LayerProperty {
    name: String,
    value: Value,
}

#[derive(Debug, Deserialize)]
pub struct Tileset {
    columns: u32,
    firstgid: u32,
    imageheight: u32,
    imagewidth: u32,
    margin: u32,
    name: String,
    spacing: u32,
    tilecount: u32,
    tileheight: u32,
    tilewidth: u32,
    // TODO: tiles
}

fn main() -> anyhow::Result<()> {
    let matches = cli::build().get_matches();

    let jitsi_host = matches.value_of("jitsi-host").unwrap();
    let workadventure_url = Url::parse(matches.value_of("workadventure-link").unwrap())?;

    let segments = workadventure_url
        .path_segments()
        .unwrap()
        .collect::<Vec<_>>();
    let instance = segments[1];
    let map_url = Url::parse(&format!("https://{}", segments[2..].join("/")))?;
    let map_base_url = Url::parse(&format!("https://{}", segments[2]))?;

    let jitsi_prefix = format!("{}{}", jitsi_host, instance);

    eprintln!("get map url... {}", map_url);

    let map_text = ureq::get(map_url.as_str()).call()?.into_string()?;
    let map = serde_json::from_str::<Map>(&map_text)?;

    let layers_with_properties = map
        .layers
        .iter()
        .filter(|o| !o.properties.is_empty())
        .collect::<Vec<_>>();

    for layer in layers_with_properties {
        inspect_layer(&workadventure_url, &map_base_url, &jitsi_prefix, &layer);
    }

    Ok(())
}

fn inspect_layer(
    workadventure_url: &Url,
    map_base_url: &Url,
    jitsi_prefix: &str,
    layer: &Layer,
) {
    println!("Layer {}", layer.name);
    for prop in &layer.properties {
        let value = parse_value_of_key(workadventure_url, map_base_url, jitsi_prefix, &layer.name, &prop.name, &prop.value)
            .unwrap_or_else(|| format!("{}", prop.value));
        println!("  {:25} {}", prop.name, value);
    }

    println!();
}

fn parse_value_of_key(
    workadventure_url: &Url,
    map_base_url: &Url,
    jitsi_prefix: &str,
    layer_name: &str,
    key: &str,
    value: &Value,
) -> Option<String> {
    match key {
        "jitsiRoom" => {
            let room = value.as_str()?;
            let jitsi_link = format!("{}{}", jitsi_prefix, room.replace('-', "").to_lowercase());
            Some(jitsi_link)
        }
        "openWebsite" | "playAudio" => {
            let url = parse_url(map_base_url, value.as_str()?);
            Some(url.into())
        }
        "startLayer" => {
            if value.as_bool()? {
            let mut url = workadventure_url.clone();
            url.set_fragment(Some(layer_name));
            Some(url.into())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_url(base_url: &Url, url_str: &str) -> Url {
    Url::parse(url_str).unwrap_or_else(|_| {
        let mut url = base_url.clone();
        url.set_path(url_str);
        url
    })
}
