//datasource_postgresql.rs
use diesel::prelude::*;

// fichier suplementiare
// mod schema; // Commenté ou supprimé
// definir les structure depuit le shema
use crate::schema;

// Implémentation de la base de données
impl Database {
    pub async fn get_database(datasource: String, cluster_id: String,) { 
        let sourceid = &datasource.sourceid;
        let database_url = &datasource.argdata;

        // ici on doit interoger la base de donner pour récupérer les cluster selon le cluster_id
        //voir les table dans shema.rs

    }

    pub async fn new_database(datasource: String, dataCluster: String,) { 
        let sourceid = &datasource.sourceid;
        let clusterid = &datacluster.clusterid;
        let database_url = &datasource.argdata;

        diesel::sql_query("CREATE TABLE IF NOT EXISTS datacluster (
            clusterid VARCHAR NOT NULL,
            datasource VARCHAR NOT NULL,
            createdate VARCHAR NOT NULL,
            editdate VARCHAR NOT NULL,
        )")

        diesel::sql_query("CREATE TABLE IF NOT EXISTS cluster_node (
            id SERIAL PRIMARY KEY,
            cluster_name VARCHAR NOT NULL,
            hostname VARCHAR NOT NULL,
            ip_address VARCHAR NOT NULL,
            control_plane BOOL NOT NULL,
            arch VARCHAR NOT NULL,
            install_disk VARCHAR NOT NULL
        )")
        .execute(&mut conn)
        .expect("Erreur lors de la création de la table des nœuds");

        diesel::sql_query("CREATE TABLE IF NOT EXISTS cluster_configuration (
            id SERIAL PRIMARY KEY,
            cluster_name VARCHAR NOT NULL,
            talos_version VARCHAR NOT NULL,
            endpoint VARCHAR NOT NULL,
            domain VARCHAR NOT NULL,
            cni_config_name VARCHAR NOT NULL
        )")
        .execute(&mut conn)
        .expect("Erreur lors de la création de la table de configuration");

    }

    pub async fn remove_cluster_database(datasource: String, datacluster: String,) { 
        let sourceid = &datasource.sourceid;
        let clusterid = &datacluster.clusterid;
        let database_url = &datasource.argdata;

        let mut conn = data.pool.get().expect("Failed to get a connection.");
        // Suppression de la source de cluster
        diesel::delete(cluster_node::table.filter(cluster_node::clusterid.eq(&clusterid)))
            .execute(&mut conn)
            .expect("Erreur lors de la suppression du cluster");
    }
    // set fonction a pour but de 
    pub async fn edit_cluster_database(datasource: String, datacluster: String,) { 
        let sourceid = &datasource.sourceid;
        let clusterid = &datacluster.clusterid;
        let database_url = &datasource.argdata;
        let mut conn = data.pool.get().expect("Failed to get a connection.");

        // Mise à jour des nœuds
        for node in nodes {
            diesel::update(cluster_node::table.filter(cluster_node::clusterid.eq(&clusterid)))
                .set((

                ))
                .execute(&mut conn)
                .expect("Erreur lors de la mise à jour du nœud");
        }

        // Mise à jour de la configuration
        diesel::update(cluster_configuration::table.filter(cluster_configuration::clusterid.eq(&clusterid)))
            .set((

            ))
            .execute(&mut conn)
            .expect("Erreur lors de la mise à jour de la configuration");

        HttpResponse::Ok().body("Cluster source updated successfully!")
    }
}
