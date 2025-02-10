use scraper::{Html, Selector};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;

fn search_google_image(query: &str) -> Option<String> {
    let url = format!("https://www.google.com/search?q={query}&tbm=isch"); // Costruiamo l'URL di ricerca immagini
    let client = Client::builder()
            .user_agent("Mozilla/5.0")      // Impostiamo un user agent per evitare il blocco di Google
            .build()
            .ok()?;

        let response = client.get(&url).send().ok()?; // Eseguiamo la richiesta GET
        let body = response.text().ok()?;   // otteniamo il corpo della risposta

        let document = Html::parse_document(&body);     // Analizzare l'HTML della pagina
        let selector = Selector::parse("img").ok()?;    // Selettore per trovare immagini

        // Troviamo il primo elemento <img> valido con un URL assoluto
        for element in document.select(&selector) {
            if let Some(src) = element.value().attr("src") {
                if src.starts_with("http") {        // Assicuriamoci che l'URL sia assoluto
                    return Some(src.to_string());
                }
            }
        }
        // Se non trova immagini valide, restituisce None
        None
}

// Funzione per scaricare un'immagine data una URL e slavarla con un nome file specificato
fn download_image(url: &str, filename: &str) -> std::io::Result<()> {
    let response = reqwest::blocking::get(url).unwrap(); // Eseguiamo la richiesta GET per l'immagine
    let bytes = response.bytes().unwrap(); // Otteniamo i byte dell'immagine
    let mut file = File::create(filename)?; // Crea un file per salvare l'immagine
    copy(&mut bytes.as_ref(), &mut file)?;  // Scrive i byte nel file
    Ok(())
}

fn main () {
    // Lista di mappe di TF2 da cercare
    let maps = vec![
    "cp_gorge_event",
    "cp_steel",
    "cp_gorge",
    "cp_carrier",
    "cp_5gorge",
    "cp_egypt_final",
    "cp_brew",
    "cp_gravelpit_snowy",
    "cp_snowplow",
    "cp_freaky_fair",
    "cp_spookeyridge",
    "cp_mountainlab",
    "cp_mossrock",
    "cp_darkmarsh",
    "cp_dustbowl",
    "cp_foundry",
    "cp_snakewater_final1",
    "cp_well",
    "cp_process_final",
    "cp_degrootkeep",
    "cp_sunshine_event",
    "cp_granary",
    "cp_coldfront",
    "cp_fastlane",
    "cp_hardwood_final",
    "cp_manor_event",
    "cp_freight_final1",
    "cp_reckoner",
    "cp_sulfur",
    "cp_gravelpit",
    "cp_vanguard",
    "cp_orange_x3",
    "cp_overgrown",
    "cp_lavapit_final",
    "cp_hadal",
    "cp_powerhouse",
    "cp_yukon_final",
    "cp_altitude",
    "cp_fortezza",
    "cp_standin_final",
    "cp_ambush_event",
    "cp_mercenarypark",
    "cp_gullywash_final1",
    "cp_junction_final",
    "cp_badlands",
    "cp_cloak",
    "cp_metalworks",
    "cp_frostwatch",
    "cp_canaveral_5cp",
    "cp_burghausen",
    "cp_sunshine",
    "cp_degrootkeep_rats"
];
    let output_dir = "./imagesControlPoints";

    // Se non esiste la cartella di output, la creiamo
    if !Path::new(output_dir).exists() {
        std::fs::create_dir(output_dir).unwrap();
    }

    // Iteriamo la lista di mappe
    for (i, map) in maps.iter().enumerate() {
        let search_query = format!("{} tf2", map); // Creiamo la query di ricerca

        if let Some(image_url) = search_google_image(&search_query) {   // Cerchiamo un'immagine
            let filename = format!("{}/{}.jpg", output_dir, map); // Creiamo il nome del file di output
            println!("Scaricando {} da {}", filename, image_url);

            // Qui bisogna scaricare l'immagine
            if let Err(e) = download_image(&image_url, &filename) {     // Scarichiamo l'immagine
                eprintln!("Errore nel download: {}", e);
            }
        } else {
            println!("Nessuna immagine trovata per {}", map);
        }
    }
}