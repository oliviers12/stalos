//edit.rs
use actix_web::{web, HttpResponse, Responder};
use serde_yaml; 

// fichier suplementiare
// mod schema; // Commenté ou supprimé
// definir les structure depuit le shema
use crate::schema;
use crate::datasource;

pub async fn edit_cluster(cluster_name: web::Path<String>) -> impl Responder {
    let cluster_name = cluster_name.into_inner();

    // Récupérer les données du cluster depuis datasource.rs
    let cluster_data = match datasource::get_cluster_source(&cluster_name).await {
        Ok(data) => data,
        Err(err) => {
            error!("Erreur lors de la récupération des données pour {}: {:?}", cluster_name, err);
            return render_error("Les données du cluster n'ont pas pu être récupérées.");
        }
    };

    // Convertir les données en YAML
    let yaml_content = match serde_yaml::to_string(&cluster_data) {
        Ok(content) => content,
        Err(err) => {
            error!("Erreur lors de la conversion en YAML pour {}: {:?}", cluster_name, err);
            return render_error("Erreur lors de la conversion en YAML.");
        }
    };

    let html = render_html(&cluster_name, &yaml_content);
    HttpResponse::Ok().content_type("text/html").body(html)
}

// Fonction pour rendre l'erreur avec un message générique
fn render_error(message: &str) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(format!(r#"
        <html>
        <body>
            <h1>Erreur</h1>
            <p>{}</p>
            <a href="{}">Retour au Menu Principal</a>
        </body>
        </html>
    "#, message, BASE_URL))
}

// Fonction pour rendre le HTML principal
fn render_html(cluster_name: &str, yaml_content: &str) -> String {
    format!(r#"
    <!DOCTYPE html>
    <html lang="fr">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Configuration Cluster</title>
        <link rel="stylesheet" href="/static/styles.css"> <!-- Lien vers le CSS externe -->
    </head>
    <body>
        <h1>Configuration Cluster: {}</h1>
        <nav>
            <a href="{}">Menu Principal</a>
            <button id="saveButton">Enregistrer</button>
        </nav>
        
        <section id="yamlEditor">
            <h2>Général</h2>
            <textarea id="yamlContent">{}</textarea>
        </section>

        <footer>
            <h2>Documentation</h2>
            <a href="https://budimanjojo.github.io/talhelper/latest/reference/configuration/">Variable</a>
        </footer>
        <script>
            document.getElementById('saveButton').addEventListener('click', async function() {{
                const yamlContent = document.getElementById('yamlContent').value;
                const response = await fetch(`{}`, {{
                    method: 'POST',
                    headers: {{ 'Content-Type': 'application/json' }},
                    body: JSON.stringify(yamlContent)
                }});
                if (response.ok) {{
                    alert('Le contenu YAML a été enregistré !');
                    window.location.reload();
                }} else {{
                    alert('Erreur lors de l\'enregistrement du YAML.');
                }}
            }});
        </script>
    </body>
    </html>
    "#, cluster_name, BASE_URL, yaml_content, YAML_SAVE_URL.replace("{}", cluster_name))
}

pub async fn save_yaml(cluster_name: web::Path<String>, yaml_content: web::Json<String>) -> impl Responder {
    let cluster_name = cluster_name.into_inner();
    let yaml_data = yaml_content.into_inner();

    // Valider le contenu YAML
    if !is_valid_yaml(&yaml_data) {
        return HttpResponse::BadRequest().body("Le contenu YAML n'est pas valide.");
    }

    // Convertir le YAML en structure Rust
    let cluster_config: Result<ClusterConfig, _> = from_str(&yaml_data);

    match cluster_config {
        Ok(config) => {
            // Appeler create_or_update_cluster avec la configuration
            match datasource::create_or_update_cluster(&cluster_name, &config, data).await {
                Ok(_) => HttpResponse::Ok().body("Données enregistrées avec succès."),
                Err(err) => {
                    error!("Erreur lors de la création du cluster: {:?}", err);
                    HttpResponse::InternalServerError().body("Erreur lors de la création du cluster.")
                }
            }
        }
        Err(err) => {
            error!("Erreur lors du parsing du YAML pour {}: {:?}", cluster_name, err);
            HttpResponse::BadRequest().body("Erreur lors du parsing du YAML.")
        }
    }
}

// Fonction de validation du YAML (exemple simpliste)
fn is_valid_yaml(yaml_data: &str) -> bool {
    // Implémentez une validation de base pour vérifier si le YAML est bien formé
    !yaml_data.trim().is_empty() // Remplacez par une logique de validation plus robuste si nécessaire
