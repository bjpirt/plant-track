use crate::types::{plant::Plant, user::User};

pub trait BaseGateway {
    async fn create_user(&self, user: &User) -> Result<(), String>;
    #[cfg(test)]
    async fn get_user(&self, user_id: &str) -> Result<User, String>;
    async fn get_user_by_username(&self, username: &String) -> Result<User, String>;
    async fn get_user_by_email(&self, email: &String) -> Result<User, String>;
    async fn create_plant(&self, plant_input: &Plant) -> Result<(), String>;
    #[cfg(test)]
    async fn get_plant(&self, plant_id: &str) -> Result<Plant, String>;
    #[cfg(test)]
    async fn get_plants_by_user(&self, user_id: &str) -> Result<Vec<Plant>, String>;
}
