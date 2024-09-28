//edit.rs
use actix_web::{web, HttpResponse, Responder};
use serde_yaml; 

// fichier suplementiare
mod schema;
// definir les structure depuit le shema
use crate::schema::*;

pub async fn edit_cluster(cluster_name: web::Path<String>) -> impl Responder {
    let cluster_name = cluster_name.into_inner();

    // Récupérer les données du cluster depuis datasource.rs
    let cluster_data = match datasource::get_cluster_source(&cluster_name).await {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::Ok().content_type("text/html").body(format!(r#"
                <html>
                <body>
                    <h1>Erreur</h1>
                    <p>Les données du cluster n'ont pas pu être récupérées.</p>
                    <a href="http://127.0.0.1:8080/">Retour au Menu Principal</a>
                </body>
                </html>
            "#));
        }
    };

    // Convertir les données en YAML
    let yaml_content = match serde_yaml::to_string(&cluster_data) {
        Ok(content) => content,
        Err(_) => "Erreur lors de la conversion en YAML.".to_string(),
    };

    let html = format!(r#"
    <!DOCTYPE html>
    <html lang="fr">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Configuration Cluster</title>
        <style>
            body {{ font-family: Arial, sans-serif; margin: 0; padding: 20px; background-color: #f4f4f4; }}
            h1, h2 {{ color: #333; }}
            nav {{ display: flex; justify-content: space-between; margin-bottom: 20px; }}
            nav a, nav button {{ padding: 10px 15px; background-color: #007BFF; color: white; border: none; border-radius: 5px; text-decoration: none; cursor: pointer; }}
            nav button:hover, nav a:hover {{ background-color: #0056b3; }}
            #yamlEditor {{ background: white; padding: 20px; border-radius: 5px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
            textarea {{ width: 100%; height: 200px; padding: 10px; border: 1px solid #ccc; border-radius: 5px; resize: none; font-family: monospace; }}
            footer {{ margin-top: 20px; text-align: center; }}
        </style>
    </head>
    <body>
        <h1>Configuration Cluster: {}</h1>
        <nav>
            <a href="http://127.0.0.1:8080/">Menu Principal</a>
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
                const response = await fetch(`/cluster/{}/save`, {{
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
    "#, cluster_name, yaml_content, cluster_name);

    HttpResponse::Ok().content_type("text/html").body(html)
}

pub async fn save_yaml(cluster_name: web::Path<String>, yaml_content: web::Json<String>) -> impl Responder {
    let cluster_name = cluster_name.into_inner();
    let yaml_data = yaml_content.into_inner();

    // Convertir le YAML en structure Rust
    let cluster_config: Result<ClusterConfig, _> = from_str(&yaml_data);

    match cluster_config {
        Ok(config) => {
            // Appeler create_or_update_cluster avec la configuration
            match datasource::create_or_update_cluster(&cluster_name, &config).await {
                Ok(_) => HttpResponse::Ok().body("Données enregistrées avec succès."),
                Err(_) => HttpResponse::InternalServerError().body("Erreur lors de l'enregistrement des données."),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Erreur lors du parsing du YAML."),
    }
}