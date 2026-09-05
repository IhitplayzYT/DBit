pub mod db_entities {
    use serde::{Serialize, Deserialize};
    use std::fs;
    use std::path::PathBuf;
    use sha2::{Sha256, Digest};
    use rand::Rng;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum EntityType {
        Schema,
        Data,
        User,
        Index,
        Table,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct DBEntity {
        pub container_name: String,
        pub entity_type: EntityType,
        pub name: String,
        pub content: String,
        pub metadata: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct DBSnapshot {
        pub container_name: String,
        pub entities: Vec<DBEntity>,
    }

    pub fn compute_db_hash(content: &str, container: &str, entity_type: &EntityType) -> String {
        let salt: u64 = rand::random();
        let mut hasher = Sha256::new();
        hasher.update(container.as_bytes());
        hasher.update(format!("{:?}", entity_type).as_bytes());
        hasher.update(salt.to_be_bytes());
        hasher.update(content.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn store_db_entity(entity: &DBEntity) -> String {
        let serialized = serde_json::to_string(entity).unwrap();
        let hash = compute_db_hash(&serialized, &entity.container_name, &entity.entity_type);
        let obj_path = PathBuf::from(".DBit/objects").join(&hash);
        if !obj_path.exists() {
            fs::write(obj_path, serialized).unwrap();
        }
        
        hash
    }

    pub fn get_db_entity(hash: &str) -> Option<DBEntity> {
        let obj_path = PathBuf::from(".DBit/objects").join(hash);
        if obj_path.exists() {
            let content = fs::read_to_string(obj_path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }

    pub fn store_snapshot(snapshot: &DBSnapshot) -> String {
        let serialized = serde_json::to_string(snapshot).unwrap();
        let salt: u64 = rand::random();
        let mut hasher = Sha256::new();
        hasher.update(snapshot.container_name.as_bytes());
        hasher.update(salt.to_be_bytes());
        hasher.update(serialized.as_bytes());
        let hash = hex::encode(hasher.finalize());
        let obj_path = PathBuf::from(".DBit/objects").join(&hash);
        if !obj_path.exists() {
            fs::write(obj_path, serialized).unwrap();
        }
        hash
    }

    pub fn get_snapshot(hash: &str) -> Option<DBSnapshot> {
        let obj_path = PathBuf::from(".DBit/objects").join(hash);
        if obj_path.exists() {
            let content = fs::read_to_string(obj_path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
}
