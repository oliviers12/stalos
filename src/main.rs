//main.rs
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::fs as std_fs;
use std::path::Path;

// fichier suplementiare
mod schema;
mod edit;
mod deploiement;
mod datasource;
mod datasource_postgresql;

// definir les structure depuit le shema
use crate::schema::*;
use crate::datasource::Database;


async fn list_clusters() -> impl Responder {
    // Initialise la variable pour stocker les clusters
    let mut list_clusters = Vec::new();

    // Récupère la liste des sources de données
    let source_list = datasource::list_source().await;

    // Itère à travers chaque source de données
    for source in source_list {
        match datasource::list_clusters(&source).await {
            Ok(clusters) => {
                for cluster in clusters {
                    list_clusters.push(cluster);
                }
            },
            Err(err) => {
                // Gestion des erreurs : ajoute un message d'erreur à la liste des clusters
                list_clusters.push(format!("Erreur avec la source {}: {:?}", source, err));
            }
        }
    }

    // Retourne la liste des clusters ou des messages d'erreur
    HttpResponse::Ok().json(list_clusters)
}


async fn create_cluster(cluster: web::Json<DataCluster>) -> impl Responder {
    let datasource = &cluster.datasource;

    match datasource::create_cluster(datasource).await {
        Ok(_) => HttpResponse::Created().body(format!("Cluster {} créé.", datasource.clusterid)),
        Err(err) => {
            error!("Erreur lors de la création du cluster: {:?}", err);
            HttpResponse::InternalServerError().body("Erreur lors de la création du cluster.")
        }
    }
}


// Function to delete a cluster
async fn delete_cluster(DataCluster: web::Path<DataCluster>) -> impl Responder {
    // Extract cluster id
    let clusterid = DataCluster.clusterid.clone();
    
    // Call the function to delete the cluster source
    let response = datasource::delete_cluster_source(clusterid.clone(), data).await;

    match response {
        Ok(_) => HttpResponse::Ok().body(format!("Cluster {} supprimé.", clusterid)),
        Err(err) => {
            error!("Erreur lors de la suppression du cluster: {:?}", err);
            HttpResponse::InternalServerError().body("Erreur lors de la suppression du cluster.")
        }
    }
}

async fn menu() -> impl Responder {
    // Liste de tous les clusters
    let cluster_list = list_clusters().await;
    // Liste de toutes les sources
    let source_list = datasource::list_source().await;

    // Construire le HTML
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>
    <html lang=\"fr\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Gestion des Clusters</title>
        <style>
            /* Styles pour la page */
            body {
                font-family: Arial, sans-serif;
            }
            .button {
                padding: 10px 15px;
                margin: 5px;
                cursor: pointer;
            }
            .header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 20px;
            }
        </style>
    </head>
    <body>

    <div class=\"header\">
        <h1>Liste des Clusters</h1>
        <a class=\"button\" href=\"/source/menu\">Gérer les Sources</a>
    </div>
    <table>
        <thead>
            <tr>
                <th>Nom du Cluster</th>
                <th>Source de Donnée</th>
                <th>Modifier</th>
                <th>Supprimer</th>
                <th>Déployer</th>
            </tr>
        </thead>
        <tbody>");

    for cluster in cluster_list {
        html.push_str(&format!(
            "<tr>
                <td>{}</td>
                <td>{}</td>
                <td><button class=\"button\" onclick=\"editCluster('{}')\">Modifier</button></td>
                <td><button class=\"button\" onclick=\"deleteCluster('{}')\">Supprimer</button></td>
                <td><a class=\"button\" href=\"/cluster/{}/deployer\">Déployer</a></td>
            </tr>",
            cluster.cluster_name, cluster.datasource, cluster.cluster_name, cluster.cluster_name, cluster.cluster_name
        ));
    }

    html.push_str(r#"</tbody>
    </table>
    
    <h2>Créer un Cluster</h2>
    <form id="createClusterForm" onsubmit="createCluster(); return false;">
        <input type="text" id="clusterName" name="cluster_name" placeholder="Nom du Cluster" required>
        <select id="dataSource" name="data_source" required>"#);

    for source in source_list {
        html.push_str(&format!("<option value=\"{}\">{}</option>", source.name, source.name));
    }

    html.push_str(r#"</select>
        <button class="button">Créer</button>
    </form>

    <h2><a href="/gestion_source">Gérer les Sources</a></h2> <!-- Lien vers la gestion des sources -->

    <script>
        async function deleteCluster(clusterid) {
            const dataSource = document.getElementById("dataSource").value;
            console.log(`Suppression du cluster : ${clusterid} dans ${dataSource}`);
            const response = await fetch(`/cluster/${dataSource}/delete`, {
                method: 'DELETE'
            });
            if (response.ok) {
                window.location.reload();
            } else {
                alert('Échec de la suppression du cluster.');
            }
        }

        async function createCluster() {
            const clusterName = document.getElementById("clusterName").value;
            const dataSource = document.getElementById("dataSource").value;
            console.log(`Création du cluster : ${clusterName} dans ${dataSource}`);
            const response = await fetch(`/cluster/${dataSource}/create`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ cluster_name: clusterName })
            });
            if (response.ok) {
                window.location.reload();
            } else {
                alert('Échec de la création du cluster.');
            }
        }

        async function editCluster(clusterName) {
            console.log(`Édition du cluster : ${clusterName}`);
            window.location.href = `/cluster/${clusterName}/edit`;
        }
    </script>
    
    </body>
    </html>"#);
    
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Route pour la liste des clusters
            .route("/", web::get().to(menu))
            .route("/cluster/{data_source}/list", web::get().to(list_cluster))
            .route("/cluster/{data_source}/create", web::post().to(create_cluster))
            .route("/cluster/{data_source}/delete", web::delete().to(delete_cluster))
            .route("/cluster/{cluster_id}/get", web::get().to(edit::get_cluster))
            .route("/cluster/{cluster_id}/edit", web::get().to(edit::edit_cluster))
            // Routes pour la gestion des sources de données
            .route("/source/menu", web::get().to(datasource::menu))
            .route("/source/list", web::post().to(datasource::list_source))
            .route("/source/create", web::post().to(datasource::create_source))
            .route("/source/delete", web::delete().to(datasource::delete_source))
            .route("/source/{data_source}/get", web::get().to(datasource::get_source))
            .route("/source/{data_source}/edit", web::post().to(datasource::edit_source))
            // Routes de déploiement (ajoutez selon vos besoins)
            // .route("/deploiement/{cluster_id}", web::post().to(deploiement::deploy_cluster))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}