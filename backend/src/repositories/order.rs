use crate::models::{Order, OrderStatus};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

use super::RepositoryError;

#[derive(Clone)]
pub struct OrderRepository {
    orders: Arc<RwLock<HashMap<Uuid, Order>>>,
}

impl OrderRepository {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn get_orders_read(&self) -> Result<RwLockReadGuard<HashMap<Uuid, Order>>, RepositoryError> {
        self.orders.read()
            .map_err(|e| RepositoryError::LockError(e.to_string()))
    }

    fn get_orders_write(&self) -> Result<RwLockWriteGuard<HashMap<Uuid, Order>>, RepositoryError> {
        self.orders.write()
            .map_err(|e| RepositoryError::LockError(e.to_string()))
    }

    pub fn add_order(&self, order: Order) -> Result<(), RepositoryError> {
        let mut orders = self.get_orders_write()?;
        if orders.contains_key(&order.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        orders.insert(order.id, order);
        Ok(())
    }

    pub fn get_order(&self, id: Uuid) -> Result<Order, RepositoryError> {
        let orders = self.get_orders_read()?;
        orders.get(&id)
            .cloned()
            .ok_or(RepositoryError::NotFound)
    }

    pub fn list_orders(&self) -> Result<Vec<Order>, RepositoryError> {
        let orders = self.get_orders_read()?;
        Ok(orders.values().cloned().collect())
    }

    pub fn add_comment(&self, id: Uuid, comment: String) -> Result<(), RepositoryError> {
        let mut orders = self.get_orders_write()?;
        let order = orders.get_mut(&id).ok_or(RepositoryError::NotFound)?;
        order.comment = Some(comment);
        Ok(())
    }

    pub fn complete_order(&self, id: Uuid) -> Result<(), RepositoryError> {
        let mut orders = self.get_orders_write()?;
        let order = orders.get_mut(&id).ok_or(RepositoryError::NotFound)?;
        order.status = OrderStatus::Completed;
        Ok(())
    }
}