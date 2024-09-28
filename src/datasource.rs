// datasource.rs
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

// fichier suplementiare
use crate::edit;
use crate::deploiement;
use crate::datasource_postgresql;
use crate::schema::*;

// ne pas oublier de crer les fonction
// create_cluster, list_source

// Structure de base de données
pub struct Database {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}


pub async fn get_all_clusters() -> Vec<Config> {
    // Appel à la fonction qui interroge la base de données
    let clusters = datasource_postgresql::get_all_clusters().await;

    // Transformer les données si nécessaire (ex : adapter les types)
    clusters
}

pub async fn create_or_update_cluster(cluster_name: &str, config: Config, data: web::Data<Database>) -> HttpResponse {
    datasource_postgresql::edit_cluster_source(cluster_name.to_string(), config, data).await;
    HttpResponse::Ok().body("Cluster updated successfully!")
}

pub async fn delete_cluster_source(cluster_name: String, data: web::Data<Database>) -> HttpResponse {
    // Appeler la fonction de suppression dans datasource_postgresql
    datasource_postgresql::remove_cluster_source(cluster_name, data).await;
    HttpResponse::Ok().body("Cluster source deleted successfully!")
}

// Fonction pour gérer et afficher les sources de données
async fn gestion_source() -> impl Responder {
    // Vérifie si le répertoire "./sources" existe, sinon le crée
    if !std_fs::metadata("./sources").is_ok() {
        std_fs::create_dir_all("./sources").unwrap();
    }
    
    // Lire le contenu du répertoire
    let paths = std_fs::read_dir("./sources").unwrap();
    let sources: Vec<String> = paths
        .filter_map(Result::ok) // Filtre les résultats valides
        .filter_map(|entry| entry.file_name().into_string().ok()) // Récupère les noms de fichiers en chaînes
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
            /* Styles pour la page */
            body {
                font-family: Arial, sans-serif;
                background-color: #f4f4f4;
                margin: 0;
                padding: 20px;
            }
            table {
                width: 100%;
                border-collapse: collapse;
                margin: 20px 0;
            }
            th, td {
                padding: 10px;
                text-align: center;
                border: 1px solid #ddd;
            }
            th {
                background-color: #4CAF50;
                color: white;
            }
            .button {
                background-color: #4CAF50;
                color: white;
                padding: 10px 15px;
                border: none;
                cursor: pointer;
            }
            .button:hover {
                background-color: #45a049;
            }
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

    // Générer une ligne de tableau pour chaque source
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

    // Ajout du formulaire pour ajouter une nouvelle source
    html.push_str(r#"</tbody>
    </table>

    <h2>Ajouter une Source</h2>
    <form id="addSourceForm" onsubmit="addSource(); return false;">
        <input type="text" id="sourceName" name="source_name" placeholder="Nom de la Source" required>
        <button class="button">Ajouter</button>
    </form>

    <script>
        // Fonction pour supprimer une source
        async function deleteSource(sourceName) {
            console.log(`Suppression de la source : ${sourceName}`);
            const response = await fetch(`/source/${sourceName}/delete`, { method: 'DELETE' });
            if (response.ok) {
                window.location.reload();
            } else {
                alert('Échec de la suppression de la source.');
            }
        }

        // Fonction pour ajouter une nouvelle source
        async function addSource() {
            const sourceName = document.getElementById("sourceName").value;
            console.log(`Ajout de la source : ${sourceName}`);
            const response = await fetch(`/source/create`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(sourceName)
            });
            if (response.ok) {
                window.location.reload();
            } else {
                alert('Échec de l\'ajout de la source.');
            }
        }

        // Fonction pour modifier une source
        async function editSource(sourceName) {
            console.log(`Édition de la source : ${sourceName}`);
            window.location.href = `/source/${sourceName}/edit`;
        }
    </script>

    </body>
    </html>"#);
    
    // Retourne la réponse HTML
    HttpResponse::Ok().content_type("text/html").body(html)
}

pub async fn create_source() -> impl Responder {
    // Implementation
}

pub async fn delete_source() -> impl Responder {
    // Implementation
}

pub async fn edit_cluster_source() -> impl Responder {
    // Implementation
}

pub async fn get_cluster_source() -> impl Responder {
    // Implementation
}
