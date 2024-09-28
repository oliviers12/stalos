use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::fs as std_fs;
use crate::datasource_postgresql;
use crate::schema::*;

// Structure de base de données
pub struct Database {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

#[derive(Deserialize)]
struct DataSource {
    sourceid: String,
    details: Config,
}

// Page web pour afficher/modifier/creer/supprimer des sources de données (PostgreSQL)
async fn menu() -> impl Responder {
    // Vérifie si le répertoire "./sources" existe, sinon le crée
    if !std_fs::metadata("./sources").is_ok() {
        std_fs::create_dir_all("./sources").unwrap();
    }
    
    // Lire le contenu du répertoire
    let paths = std_fs::read_dir("./sources").unwrap();
    let sources: Vec<String> = paths
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    // Construire le HTML pour afficher les sources
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>
    <html lang=\"fr\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Gestion des Sources</title>
        <style>
            body { font-family: Arial, sans-serif; background-color: #f4f4f4; margin: 0; padding: 20px; }
            table { width: 100%; border-collapse: collapse; margin: 20px 0; }
            th, td { padding: 10px; text-align: center; border: 1px solid #ddd; }
            th { background-color: #4CAF50; color: white; }
            .button { background-color: #4CAF50; color: white; padding: 10px 15px; border: none; cursor: pointer; }
            .button:hover { background-color: #45a049; }
        </style>
    </head>
    <body>
    <h1>Liste des Sources de Données</h1>
    <table>
        <thead>
            <tr>
                <th>Nom de la Source</th>
                <th>Modifier</th>
                <th>Supprimer</th>
            </tr>
        </thead>
        <tbody>");

    for source in sources {
        html.push_str(&format!(
            "<tr>
                <td>{}</td>
                <td><button class=\"button\" onclick=\"editSource('{}')\">Modifier</button></td>
                <td><button class=\"button\" onclick=\"deleteSource('{}')\">Supprimer</button></td>
            </tr>",
            source, source, source
        ));
    }

    html.push_str(r#"</tbody>
    </table>
    <h2>Ajouter une Source</h2>
    <form id="addSourceForm" onsubmit="addSource(); return false;">
        <input type="text" id="sourceName" name="source_name" placeholder="Nom de la Source" required>
        <button class="button">Ajouter</button>
    </form>
    <script>
        async function deleteSource(sourceName) {
            const response = await fetch(`/source/${sourceName}/delete`, { method: 'DELETE' });
            if (response.ok) { window.location.reload(); } else { alert('Échec de la suppression de la source.'); }
        }
        async function addSource() {
            const sourceName = document.getElementById("sourceName").value;
            const response = await fetch(`/source/create`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ sourceid: sourceName })
            });
            if (response.ok) { window.location.reload(); } else { alert('Échec de l\'ajout de la source.'); }
        }
        async function editSource(sourceName) { window.location.href = `/source/${sourceName}/edit`; }
    </script>
    </body>
    </html>"#);

    HttpResponse::Ok().content_type("text/html").body(html)
}

pub async fn list_source() -> HttpResponse {
    // Code pour lister toutes les sources de données à partir de source.json
    // À implémenter
    HttpResponse::Ok().body("Liste de toutes les sources de données")
}

pub async fn create_source(source: web::Json<DataSource>, data: web::Data<Database>) -> HttpResponse {
    // Code pour ajouter une source de données et rediriger vers edit_source
    // À implémenter
    HttpResponse::Ok().body("Source de données créée")
}

pub async fn delete_source(sourceid: web::Path<String>, data: web::Data<Database>) -> HttpResponse {
    // Code pour supprimer une source de données
    // À implémenter
    HttpResponse::Ok().body("Source de données supprimée")
}

pub async fn get_source(sourceid: web::Path<String>, data: web::Data<Database>) -> HttpResponse {
    // Code pour obtenir une source de données spécifique
    // À implémenter
    HttpResponse::Ok().body("Détails de la source de données")
}

pub async fn edit_source(sourceid: web::Path<String>, source: web::Json<DataSource>, data: web::Data<Database>) -> HttpResponse {
    // Code pour modifier les paramètres d'une source de données
    // À implémenter
    HttpResponse::Ok().body("Source de données modifiée")
}

pub async fn create_cluster(cluster: web::Json<DataSource>, data: web::Data<Database>) -> HttpResponse {
    let datasource = &cluster.sourceid;
    let config = &cluster.details;

    match datasource_postgresql::create_cluster(datasource, config, data).await {
        Ok(_) => HttpResponse::Created().body(format!("Cluster {} créé.", config.cluster_name)),
        Err(err) => {
            error!("Erreur lors de la création du cluster: {:?}", err);
            HttpResponse::InternalServerError().body("Erreur lors de la création du cluster.")
        }
    }
}

pub async fn edit_cluster(cluster: web::Json<DataSource>, data: web::Data<Database>) -> HttpResponse {
    let datasource = &cluster.sourceid;
    let config = &cluster.details;

    match datasource_postgresql::edit_cluster_source(datasource, config, data).await {
        Ok(_) => HttpResponse::Ok().body("Cluster modifié avec succès."),
        Err(err) => {
            error!("Erreur lors de la modification du cluster: {:?}", err);
            HttpResponse::InternalServerError().body("Erreur lors de la modification du cluster.")
        }
    }
}

pub async fn list_cluster(data: web::Data<Database>) -> HttpResponse {
    // Code pour lister tous les clusters de toutes les sources
    // À implémenter
    HttpResponse::Ok().body("Liste de tous les clusters")
}