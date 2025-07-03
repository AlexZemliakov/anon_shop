use crate::models::Product;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

use super::RepositoryError;

#[derive(Clone)]
pub struct ProductRepository {
    products: Arc<RwLock<HashMap<Uuid, Product>>>,
}

impl ProductRepository {
    pub fn new() -> Self {
        Self {
            products: Arc::new(RwLock::new(HashMap::new())),
        }
    }


    fn get_products_read(&self) -> Result<RwLockReadGuard<HashMap<Uuid, Product>>, RepositoryError> {
        self.products.read()
            .map_err(|e| RepositoryError::LockError(e.to_string()))
    }

    fn get_products_write(&self) -> Result<RwLockWriteGuard<HashMap<Uuid, Product>>, RepositoryError> {
        self.products.write()
            .map_err(|e| RepositoryError::LockError(e.to_string()))
    }

    pub fn add_product(&self, product: Product) -> Result<(), RepositoryError> {
        let mut products = self.get_products_write()?;

        if products.contains_key(&product.id) {
            return Err(RepositoryError::AlreadyExists);
        }

        products.insert(product.id, product);
        Ok(())
    }

    pub fn update_product(&self, product: Product) -> Result<(), RepositoryError> {
        let mut products = self.get_products_write()?;

        if !products.contains_key(&product.id) {
            return Err(RepositoryError::NotFound);
        }

        products.insert(product.id, product);
        Ok(())
    }

    pub fn get_product(&self, id: Uuid) -> Result<Product, RepositoryError> {
        let products = self.get_products_read()?;
        products.get(&id)
            .cloned()
            .ok_or(RepositoryError::NotFound)
    }

    pub fn list_products(&self) -> Result<Vec<Product>, RepositoryError> {
        let products = self.get_products_read()?;
        Ok(products.values().cloned().collect())
    }

    pub fn remove_product(&self, id: Uuid) -> Result<Product, RepositoryError> {
        let mut products = self.get_products_write()?;
        products.remove(&id)
            .ok_or(RepositoryError::NotFound)
    }
}