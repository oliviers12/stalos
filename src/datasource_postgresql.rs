//datasource_postgresql.rs
use diesel::prelude::*;

// fichier suplementiare
mod schema;

// definir les structure depuit le shema
use crate::schema::*;

// Implémentation de la base de données
impl Database {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
        Database { pool }
    }

    // Création des tables
    pub fn create_tables(&self) {
        let mut conn = self.pool.get().expect("Failed to get a connection.");

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

    // datasource_postgresql.rs
    pub async fn remove_cluster_source(cluster_name: String, data: web::Data<Database>) {
        let mut conn = data.pool.get().expect("Failed to get a connection.");
        // Suppression de la source de cluster
        diesel::delete(cluster_node::table.filter(cluster_node::cluster_name.eq(&cluster_name)))
            .execute(&mut conn)
            .expect("Erreur lors de la suppression du cluster");
    }

    pub async fn get_all_clusters() -> Vec<Config> {
        use crate::schema::cluster_configuration::dsl::*;
    
        // Connexion à la base de données
        let connection = establish_connection().await; // Implémentez cette fonction selon votre setup
    
        // Interroger la base de données
        let results = connection
            .run(move |conn| {
                cluster_configuration
                    .load::<Config>(conn)
            })
            .await
            .expect("Erreur lors de la récupération des clusters");
    
        results
    }

    // Mise à jour d'une source de cluster
    pub async fn edit_cluster_source(web::Path(cluster_name): web::Path<String>, json: web::Json<Config>, data: web::Data<Database>) -> HttpResponse {
        let mut conn = data.pool.get().expect("Failed to get a connection.");

        // Mise à jour des nœuds
        for node in &json.nodes {
            diesel::update(cluster_node::table.filter(cluster_node::cluster_name.eq(&cluster_name)))
                .set((
                    cluster_node::hostname.eq(&node.hostname),
                    cluster_node::ip_address.eq(&node.ip_address),
                    cluster_node::control_plane.eq(node.control_plane),
                    cluster_node::arch.eq(&node.arch),
                    cluster_node::install_disk.eq(&node.install_disk),
                ))
                .execute(&mut conn)
                .expect("Erreur lors de la mise à jour du nœud");
        }

        // Mise à jour de la configuration
        diesel::update(cluster_configuration::table.filter(cluster_configuration::cluster_name.eq(&cluster_name)))
            .set((
                cluster_configuration::talos_version.eq(&json.talos_version),
                cluster_configuration::endpoint.eq(&json.endpoint),
                cluster_configuration::domain.eq(&json.domain),
                cluster_configuration::cni_config_name.eq(&json.cni_config.name),
            ))
            .execute(&mut conn)
            .expect("Erreur lors de la mise à jour de la configuration");

        HttpResponse::Ok().body("Cluster source updated successfully!")
    }

    // Récupération d'une source de cluster
    pub async fn get_cluster_source(web::Path(cluster_name): web::Path<String>, data: web::Data<Database>) -> HttpResponse {
        let mut conn = data.pool.get().expect("Failed to get a connection.");

        // Récupérer les nœuds
        let nodes: Vec<ClusterNode> = cluster_node::table
            .filter(cluster_node::cluster_name.eq(&cluster_name))
            .load(&mut conn)
            .expect("Erreur lors de la récupération des nœuds");

        // Récupérer la configuration
        let configuration: ClusterConfiguration = cluster_configuration::table
            .filter(cluster_configuration::cluster_name.eq(&cluster_name))
            .first(&mut conn)
            .expect("Erreur lors de la récupération de la configuration");

        let response = ClusterSource {
            nodes,
            configuration,
        };

        HttpResponse::Ok().json(response)
    }
}
