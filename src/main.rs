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

async fn create_cluster(cluster_name: web::Json<String>, data: web::Data<Database>) -> impl Responder {
    // Récupération des données de configuration
    let cluster_data = datasource::get_cluster_data(&cluster_name).await;

    // Création de la configuration par défaut
    let default_cluster = Config {
        cluster_name: cluster_name.clone(),
        talos_version: cluster_data.talos_version.unwrap_or("v1.7.6".to_string()),
        endpoint: format!("https://{}:6443", cluster_name),
        domain: "domain.local".to_string(),
        cni_config: CniConfig { name: "none".to_string() },
        nodes: cluster_data.nodes,
    };

    // Appeler la fonction pour créer ou mettre à jour le cluster
    let response = datasource::create_or_update_cluster(&cluster_name, default_cluster, data).await;


    HttpResponse::Created().body(format!("Cluster {} créé.", cluster_name))
}


async fn delete_cluster(cluster_name: web::Path<String>, data: web::Data<Database>) -> impl Responder {
    // Appeler la fonction pour supprimer la source du cluster
    let response = datasource::remove_cluster_source(cluster_name.into_inner(), data).await;

    HttpResponse::Ok().body(format!("Cluster {} supprimé.", cluster_name))
}

async fn list_clusters() -> impl Responder {
    // Récupération des données de configuration
    let clusters = datasource::get_all_clusters().await;

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
        <a class=\"button\" href=\"/gestion_source\">Gérer les Sources</a>
    </div>
    <table>
        <thead>
            <tr>
                <th>Nom du Cluster</th>
                <th>Modifier</th>
                <th>Supprimer</th>
                <th>Déployer</th>
            </tr>
        </thead>
        <tbody>");

    for cluster in clusters {
        html.push_str(&format!(
            "<tr>
                <td>{}</td>
                <td><button class=\"button\" onclick=\"editCluster('{}')\">Modifier</button></td>
                <td><button class=\"button\" onclick=\"deleteCluster('{}')\">Supprimer</button></td>
                <td><a class=\"button\" href=\"/cluster/{}/deployer\">Déployer</a></td>
            </tr>",
            cluster.cluster_name, cluster.cluster_name, cluster.cluster_name, cluster.cluster_name
        ));
    }

    html.push_str(r#"</tbody>
    </table>
    
    <h2>Créer un Cluster</h2>
    <form id="createClusterForm" onsubmit="createCluster(); return false;">
        <input type="text" id="clusterName" name="cluster_name" placeholder="Nom du Cluster" required>
        <button class="button">Créer</button>
    </form>

    <h2><a href="/gestion_source">Gérer les Sources</a></h2> <!-- Lien vers la gestion des sources -->

    <script>
        async function deleteCluster(clusterName) {
            console.log(`Suppression du cluster : ${clusterName}`);
            const response = await fetch(`/cluster/${clusterName}/delete`, { method: 'DELETE' });
            if (response.ok) {
                window.location.reload();
            } else {
                alert('Échec de la suppression du cluster.');
            }
        }

        async function createCluster() {
            const clusterName = document.getElementById("clusterName").value;
            console.log(`Création du cluster : ${clusterName}`);
            const response = await fetch(`/cluster/create`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(clusterName)
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
            .route("/", web::get().to(list_clusters))
            .route("/cluster/create", web::post().to(create_cluster))
            .route("/cluster/{cluster_name}/delete", web::delete().to(delete_cluster))
            .route("/cluster/{cluster_name}/edit", web::get().to(edit::edit_cluster))
            .route("/cluster/{cluster_name}/deployer", web::get().to(deploiement::deploy_talos))
            .route("/gestion_source", web::get().to(datasource::gestion_source))
            .route("/source/create", web::post().to(datasource::create_source))
            .route("/source/{source_name}/delete", web::delete().to(datasource::delete_source))
            .route("/source/{cluster_name}/edit", web::post().to(datasource::edit_cluster_source))
            .route("/source/{cluster_name}/get", web::post().to(datasource::get_cluster_source))
            // Ajoutez d'autres routes selon vos besoins
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}