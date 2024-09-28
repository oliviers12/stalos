use diesel::{table, Insertable};
use serde::{Deserialize, Serialize};

// Définition de la table des nœuds de cluster
table! {
    cluster_node (id) {
        id -> Int4,  // Identifiant unique du nœud
        cluster_name -> Varchar,  // Nom du cluster auquel appartient le nœud
        hostname -> Varchar,  // Nom d'hôte du nœud
        ip_address -> Varchar,  // Adresse IP du nœud
        control_plane -> Bool,  // Indique si le nœud fait partie du plan de contrôle
        arch -> Varchar,  // Architecture du nœud
        install_disk -> Varchar,  // Disque d'installation du nœud
    }
}

// Définition de la table de configuration du cluster
table! {
    cluster_configuration (id) {
        id -> Int4,  // Identifiant unique de la configuration
        cluster_name -> Varchar,  // Nom du cluster
        talos_version -> Varchar,  // Version de Talos utilisée
        endpoint -> Varchar,  // Point de terminaison du cluster
        domain -> Varchar,  // Domaine du cluster
        cni_config_name -> Varchar,  // Nom de la configuration CNI
    }
}

// Définition de la table de datacluster
table! {
    datacluster (clusterid) {
        clusterid -> Varchar,  // Nom du cluster
        datasource -> Varchar,  // emplacement ou les données sont stockées
        createdate -> Varchar,  // date de création du cluster
        editdate -> Varchar,  // date de dernière modification du cluster
    }
}

// Structure pour configurer un datacluster
#[derive(Serialize, Deserialize, Insertable)]
pub struct Datasource {
    sourceid: String,  // Nom du source
    typedata: String,  // le type comme posgrsql
    argdata: String, // data de la source comme l'url posgrsql
    createdate: String,  // date de création du source
    editdate: String,  // date de dernière modification de la configuration de source

}

// Structure pour configurer un datacluster
#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = datacluster)]
pub struct DataCluster {
    clusterid: String,  // Nom du cluster
    datasource: String,  // emplacement ou les données sont stockées
    createdate: String,  // date de création du cluster
    editdate: String,  // date de dernière modification du cluster
    clusters: Vec<Cluster>,  // Structures pour représenter un cluster
}

// Structure pour configurer un cluster
#[derive(Serialize, Deserialize)]
pub struct Cluster {
    cluster_name: String,  // Nom du cluster
    talos_version: String,  // Version de Talos
    endpoint: String,  // Point de terminaison
    domain: String,  // Domaine
    cni_config: CniConfig,  // Configuration CNI
    nodes: Vec<Node>,  // Liste des nœuds
}

// Structure pour la configuration CNI
#[derive(Serialize, Deserialize)]
pub struct CniConfig {
    name: String,  // Nom de la configuration CNI
}

// Structure représentant un nœud
#[derive(Serialize, Deserialize)]
pub struct Node {
    hostname: String,  // Nom d'hôte du nœud
    ip_address: String,  // Adresse IP
    control_plane: bool,  // Indicateur de plan de contrôle
    arch: String,  // Architecture
    install_disk: String,  // Disque d'installation
    nameservers: Vec<String>,  // Liste des serveurs de noms
    network_interfaces: Vec<NetworkInterface>,  // Interfaces réseau
}

// Structure représentant une interface réseau
#[derive(Serialize, Deserialize)]
pub struct NetworkInterface {
    device_selector: DeviceSelector,  // Sélecteur de périphérique
    addresses: Vec<String>,  // Liste d'adresses
    routes: Vec<Route>,  // Routes associées
}

// Structure représentant une route
#[derive(Serialize, Deserialize)]
pub struct Route {
    network: String,  // Réseau de la route
    gateway: String,  // Passerelle de la route
}

// Structure représentant un sélecteur de périphérique
#[derive(Serialize, Deserialize)]
pub struct DeviceSelector {
    driver: String,  // Driver du périphérique
}